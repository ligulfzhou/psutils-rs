use crate::client::Client;
use crate::errors::PSError;

#[derive(Debug)]
pub struct Load {
    pub load1: f32,
    pub load5: f32,
    pub load15: f32,
}

impl Client {
    pub fn load(&mut self) -> Result<Load, PSError> {
        let res = self.content("/proc/loadavg")?;

        let sp: Vec<&str> = res.split(' ').collect();
        if sp.len() < 3 {
            return Err(PSError::CustomError("not likely to happen, /proc/loadavg"));
        }
        Ok(Load {
            load1: sp.get(0).unwrap().parse::<f32>().unwrap(),
            load5: sp.get(1).unwrap().parse::<f32>().unwrap(),
            load15: sp.get(2).unwrap().parse::<f32>().unwrap(),
        })
    }
}
