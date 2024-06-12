use std::fs;
use serde_derive::{Deserialize, Serialize};
use toml::de::Error;
#[derive(Serialize,Deserialize)]
pub struct Config{
    pub cpu: Option<Vec<CPU>>,
    pub memory: Option<f32>,
}
impl Config{
    pub fn load(file:String) -> Result<Config, Error> {
        let contents = fs::read_to_string(file).expect("Failed to read config file");
        toml::from_str(&contents)
    }
}
#[derive(Serialize,Deserialize)]
pub struct CPU{
    // core index number
    pub id: u32,
    // cpu usage by %
    pub limit: Option<f32>,
    pub jitter: Option<f32>,
    pub time: Option<u64>
}
impl CPU{
}