use std::time::Duration;

use tokio::{signal, time};

use crate::Rpi;

pub struct App {
    tick_interval: Duration,
    platform: Rpi,
}

impl App {
    pub fn new() -> Self {
        Self {
            platform: Rpi::default(),
            tick_interval: Duration::from_millis(500),
        }
    }

    pub fn with_tick_duration(mut self, duration: Duration) -> Self {
        self.tick_interval = duration;
        self
    }

    pub async fn run(&mut self) {
        let mut interval = time::interval(self.tick_interval);

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    self.platform.update().unwrap_or_else(|_|todo!());
                    self.draw().await
                }
                _ = signal::ctrl_c()=>{
                    println!("Ctrl-C signal received, shutting down.");
                    break;
                }
            }
        }
    }

    async fn draw(&self) {
        println!("Data:-------------------------");
        println!("CPU CLK : {:}", self.platform.cpu.clock.arm);
        println!("GPU CLK : {:}", self.platform.cpu.clock.gpu);
        println!("CPU TEMP: {:}", *self.platform.cpu.temp);
        println!("FAN PWM : {:}", self.platform.fan.pwm);
        println!("FAN RPM : {:}", self.platform.fan.rpm);
        println!("THROTTLE: {:?}", self.platform.power.throttle.current);
        println!("THROTHAS: {:?}", self.platform.power.throttle.happened);
        println!(
            "POWER   : {:}",
            self.platform
                .power
                .power
                .power_map
                .iter()
                .map(|m| m.amps * m.volts)
                .sum::<f32>()
        );
        println!("Log:--------------------------");
        println!("");
    }
}
