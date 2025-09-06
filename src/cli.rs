use std::{ops::Sub, time::Duration};

use crossterm::event::{Event, EventStream, KeyCode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{Axis, Block, Chart, Dataset, Paragraph, Wrap},
    DefaultTerminal, Frame,
};
use tokio::time::{self, Interval};

use crate::{Result, Rpi};
use tokio_stream::StreamExt;

pub struct App {
    quit: bool,
    tick_interval: Interval,
    // ui_refresh_interval: Duration,
    platform: Rpi,
    chart_data: Vec<f64>,
    chart_bounds: [f64; 2],
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    // const FRAMES_PER_SECOND: u64 = 60;
    // const MICROS_IN_SECOND: u64 = 1_000_000;

    pub fn new() -> Self {
        Self {
            tick_interval: time::interval(Duration::from_millis(1000)),
            // ui_refresh_interval: Duration::from_micros(
            //     App::MICROS_IN_SECOND / App::FRAMES_PER_SECOND,
            // ),
            quit: false,
            platform: Default::default(),
            chart_data: vec![0.0f64; 200],
            chart_bounds: [0f64, 200f64],
        }
    }

    pub fn with_tick_duration(mut self, duration: Duration) -> Self {
        self.tick_interval = time::interval(duration);
        self
    }

    pub async fn run(&mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        // let mut tick_interval = time::interval(self.tick_interval);
        // let mut ui_refresh_interval = time::interval(self.ui_refresh_interval);
        let mut events = EventStream::new();

        while !self.quit {
            tokio::select! {
                Some(Ok(event)) = events.next() => {
                    self.handle_event(&event);
                },
                _ = self.tick_interval.tick() => {
                    self.on_tick()?;
                    terminal.draw(|frame| self.render(frame))?;
                },
                // FIXME Not needed if it utilises too much resources
                // _ = ui_refresh_interval.tick() => {
                //     // self.platform.update()?;
                //     terminal.draw(|frame| self.render(frame))?;
                // }
            }
        }
        Ok(())
    }

    fn on_tick(&mut self) -> Result<()> {
        self.platform.update()?;
        self.chart_update();
        Ok(())
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
        let one_sec = Duration::from_secs(1);
        let five_hundred_ms = Duration::from_millis(500);
        let hundred_ms = Duration::from_millis(100);
        let current_period = self.tick_interval.period();

        self.tick_interval = match current_period >= one_sec {
            true => time::interval(current_period.saturating_add(five_hundred_ms)),
            false => time::interval(current_period.saturating_add(hundred_ms)),
        }
    }

    fn decrease_interval(&mut self) {
        let one_and_half_sec = Duration::from_millis(1500);
        let five_hundred_ms = Duration::from_millis(500);
        let hundred_ms = Duration::from_millis(100);
        let current_period = self.tick_interval.period();

        self.tick_interval = match current_period >= one_and_half_sec {
            true => time::interval(current_period.sub(five_hundred_ms)),
            false => match current_period <= hundred_ms {
                true => time::interval(hundred_ms),
                false => time::interval(current_period.sub(hundred_ms)),
            },
        }
    }

    fn chart_update(&mut self) {
        self.chart_data.drain(0..1);
        self.chart_data
            .push(self.platform.power.power.total_power.into());
    }
}

/// Rendering implementations ONLY
impl App {
    fn render(&self, frame: &mut Frame) {
        let [main_block, chart_block] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Max(8), Constraint::Max(15)])
            .areas(frame.area());
        let [main_block, throtte_block] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)])
            .areas(main_block);

        self.render_main_area(frame, main_block);
        self.render_throttle_area(frame, throtte_block);
        self.render_animated_chart(frame, chart_block);
    }

    fn render_animated_chart(&self, frame: &mut Frame, area: Rect) {
        let set = self
            .chart_data
            .iter()
            .enumerate()
            .map(|(i, pow)| (i as f64, *pow))
            .collect::<Vec<_>>();
        let datasets = vec![Dataset::default()
            .name(format!("{:.3?}W", &set.last().unwrap_or(&(0f64, 0f64)).1))
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Cyan))
            .data(&set)];

        let chart = Chart::new(datasets)
            .block(Block::bordered())
            .x_axis(
                Axis::default()
                    .labels(["Past", "Now"])
                    .bounds(self.chart_bounds),
            )
            .y_axis(
                Axis::default()
                    .title("Power(W)")
                    .labels(["0", "5", "10"])
                    .bounds([0.0, 10.0]),
            );

        frame.render_widget(chart, area);
    }

    fn render_main_area(&self, frame: &mut Frame, area: Rect) {
        let text = vec![
            Line::from(format!("CPU TEMP   : {:.1}", *self.platform.cpu.temp)),
            Line::from(format!("ARM CLOCK  : {:.2}", *self.platform.cpu.clock.arm)),
            Line::from(format!("GPU CLOCK  : {:.2}", *self.platform.cpu.clock.gpu)),
            Line::from(format!("FAN PWM    : {}", self.platform.fan.pwm)),
            Line::from(format!("FAN RPM    : {}", self.platform.fan.rpm)),
            Line::from(format!(
                "TOTAL POWER: {:<6.3}",
                &self.platform.power.power.total_power
            )),
        ];

        let right_title = Line::from(vec![
            Span::styled(" - ", Style::new().red()),
            Span::raw(format!(" {:?} ", self.tick_interval.period())),
            Span::styled(" + ", Style::new().green()),
        ])
        .right_aligned()
        .bold();

        let main_paragraph = Paragraph::new(text)
            .block(Block::bordered().title(right_title))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        frame.render_widget(main_paragraph, area);
    }

    fn render_throttle_area(&self, frame: &mut Frame, area: Rect) {
        let [current_throttle_block, previous_throtte_block] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(area);
        let current_throttle_text = vec![
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
        ];
        let previous_throttle_text = vec![
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

        let current_throttle_paragraph = Paragraph::new(current_throttle_text)
            .block(Block::bordered().title(Line::from("Current throttle:")))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        let previous_throttle_paragraph = Paragraph::new(previous_throttle_text)
            .block(Block::bordered().title(Line::from("Previous throttle:")))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        frame.render_widget(current_throttle_paragraph, current_throttle_block);
        frame.render_widget(previous_throttle_paragraph, previous_throtte_block);
    }
}
