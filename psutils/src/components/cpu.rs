use crate::client::Client;
use crate::errors::PSResult;

#[derive(Debug)]
struct CpuTimeStat {
    cpu: String,
    user: f32,
    system: f32,
    idle: f32,
    nice: f32,
    iowait: f32,
    irq: f32,
    soft_irq: f32,
    steal: f32,
    guest: f32,
    guest_nice: f32,
}

impl Client {
    pub fn cpu_count(&self) -> PSResult<usize> {
        Ok(4)
    }

    pub fn cpu_info() {}
}
