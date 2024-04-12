#[derive(Default)]
pub struct CPU{
    // core index number
    pub core_id: u32,
    // cpu usage by %
    pub limit: f32,
    pub jitter: f32,
}
impl CPU{
    pub fn from_config(config_str:&str)->CPU{
        let _part = config_str.trim();
        let _part:Vec<&str> = _part.split("/").collect();
        let core = _part[0].parse::<u32>().unwrap();
        let limit = _part.get(1).unwrap_or(&"1").parse::<f32>().unwrap_or(1.0);
        let jitter:f32 = _part.get(2).unwrap_or(&"0").parse::<f32>().unwrap_or(0.0);
        CPU{
            core_id: core,
            limit,
            jitter,
        }
    }
}