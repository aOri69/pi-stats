mod command;
mod cpu;
mod fan;
mod power;

use crate::{
    platform::{cpu::CpuStatus, fan::FanStatus, power::PowerStatus},
    Result,
};

#[derive(Default)]
pub struct Rpi {
    pub cpu: CpuStatus,
    pub fan: FanStatus,
    pub power: PowerStatus,
}

impl Rpi {
    pub fn update(&mut self) -> Result<()> {
        self.cpu = CpuStatus::new()?;
        self.fan = FanStatus::new()?;
        self.power = PowerStatus::new()?;
        Ok(())
    }
}
