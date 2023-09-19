use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ChaosConfig {
    pub tx: Option<f32>,
    pub send: Option<f32>,
    pub get_log: Option<f32>,
    pub stream_log: Option<f32>,
}

impl ChaosConfig {
    pub fn new(
        tx: Option<f32>,
        send: Option<f32>,
        get_log: Option<f32>,
        stream_log: Option<f32>,
    ) -> Self {
        Self { tx, send, get_log, stream_log }
    }
}

impl FromStr for ChaosConfig {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tx = None;
        let mut send = None;
        let mut get_log = None;
        let mut stream_log = None;

        let configured: Vec<&str> = s.split(',').collect();
        for config in configured {
            let components: Vec<&str> = config.split(':').collect();
            let key = components[0].trim();
            let value = match components.len() {
                1 => 0.01, // No value specified, default to 0.01
                2 => {
                    let value_str = components[1].trim();
                    value_str
                        .parse::<f32>()
                        .map_err(|_| format!("Failed to parse value: {}", value_str))?
                }
                _ => return Err(format!("Invalid chaos config: {}", config)),
            };

            if value < 0.0 || value > 1.0 {
                return Err(format!("Value out of range [0, 1]: {}", value))
            }

            match key {
                "tx" => tx = Some(value),
                "send" => send = Some(value),
                "get_log" => get_log = Some(value),
                "stream_log" => stream_log = Some(value),
                _ => return Err(format!("Unrecognized key: {}", key)),
            }
        }
        Ok(Self::new(tx, send, get_log, stream_log))
    }
}
