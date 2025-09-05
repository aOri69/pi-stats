use std::time::Duration;

use crossterm::event::{Event, EventStream, KeyCode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
    DefaultTerminal, Frame,
};
use tokio::time;

use crate::Rpi;
use tokio_stream::StreamExt;

pub struct App {
    quit: bool,
    tick_interval: Duration,
    ui_refresh_interval: Duration,
    platform: Rpi,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    const FRAMES_PER_SECOND: u64 = 60;
    const MICROS_IN_SECOND: u64 = 1_000_000;

    pub fn new() -> Self {
        Self {
            tick_interval: Duration::from_millis(500),
            ui_refresh_interval: Duration::from_micros(
                App::MICROS_IN_SECOND / App::FRAMES_PER_SECOND,
            ),
            quit: false,
            platform: Default::default(),
        }
    }

    pub fn with_tick_duration(mut self, duration: Duration) -> Self {
        self.tick_interval = duration;
        self
    }

    pub async fn run(&mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        let mut tick_interval = time::interval(self.tick_interval);
        let mut ui_refresh_interval = time::interval(self.ui_refresh_interval);
        let mut events = EventStream::new();

        while !self.quit {
            tokio::select! {
                Some(Ok(event)) = events.next() => {
                    self.handle_event(&event)
                },
                _ = tick_interval.tick() => {
                    self.platform.update()?;
                    // terminal.draw(|frame| self.render(frame))?;
                },
                _ = ui_refresh_interval.tick() => {
                    // self.platform.update()?;
                    terminal.draw(|frame| self.render(frame))?;
                }
            }
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let [main_block] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(100)])
            .areas(frame.area());

        let text = vec![
            Line::from(format!("CPU TEMP   : {:.1}", *self.platform.cpu.temp)),
            Line::from(format!("ARM CLOCK  : {:.2}", *self.platform.cpu.clock.arm)),
            Line::from(format!("GPU CLOCK  : {:.2}", *self.platform.cpu.clock.gpu)),
            Line::from(format!("FAN PWM    : {}", self.platform.fan.pwm)),
            Line::from(format!("FAN RPM    : {}", self.platform.fan.rpm)),
            Line::from(format!(
                "TOTAL POWER: {:<6.3}",
                &self
                    .platform
                    .power
                    .power
                    .power_map
                    .iter()
                    .map(|measure| measure.volts * measure.amps)
                    .sum::<f32>()
            )),
            // Current throttle
            Line::from("CURRENT THROTTLE    :"),
            Line::from(format!(
                "ARM FREQUENCY CAPPED: {}",
                self.platform.power.throttle.current.arm_frequency_capped
            )),
            Line::from(format!(
                "SOFT TEMP LIMIT     : {}",
                self.platform.power.throttle.current.soft_temp_limit
            )),
            Line::from(format!(
                "THROTTLED           : {}",
                self.platform.power.throttle.current.throttled
            )),
            Line::from(format!(
                "UNDER VOLTAGE       : {}",
                self.platform.power.throttle.current.under_voltage
            )),
            // Previous throttle
            Line::from("PREVIOUS THROTTLE   :"),
            Line::from(format!(
                "ARM FREQUENCY CAPPED: {}",
                self.platform.power.throttle.happened.arm_frequency_capped
            )),
            Line::from(format!(
                "SOFT TEMP LIMIT     : {}",
                self.platform.power.throttle.happened.soft_temp_limit
            )),
            Line::from(format!(
                "THROTTLED           : {}",
                self.platform.power.throttle.happened.throttled
            )),
            Line::from(format!(
                "UNDER VOLTAGE       : {}",
                self.platform.power.throttle.happened.under_voltage
            )),
        ];

        // for va_char in &self.platform.power.power.power_map {
        //     text.push(Line::from(format!(
        //         "| {:<9} | {:<5.2}V | {:<5.2}A | {:<6.3}W |",
        //         va_char.measure,
        //         va_char.volts,
        //         va_char.amps,
        //         va_char.volts * va_char.amps
        //     )));
        // }

        let right_title = Line::from(vec![
            Span::styled(" - ", Style::new().red()),
            Span::raw(format!(" {:?} ", self.tick_interval)),
            Span::styled(" + ", Style::new().green()),
        ])
        .right_aligned()
        .bold();

        let main_paragraph = Paragraph::new(text)
            .block(Block::bordered().title(right_title))
            .style(Style::new().white().on_black())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        frame.render_widget(main_paragraph, main_block);
    }

    fn handle_event(&mut self, event: &Event) {
        if let Some(key) = event.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.quit = true,
                KeyCode::Char('c')
                    if key
                        .modifiers
                        .contains(crossterm::event::KeyModifiers::CONTROL) =>
                {
                    self.quit = true
                }
                KeyCode::Char('+') => self.increase_interval(),
                KeyCode::Char('-') => self.decrease_interval(),
                _ => {}
            }
        }
    }

    fn increase_interval(&mut self) {
        self.tick_interval = self
            .tick_interval
            .saturating_add(Duration::from_millis(500));
    }

    fn decrease_interval(&mut self) {
        self.tick_interval = self
            .tick_interval
            .saturating_sub(Duration::from_millis(500));
    }
}
