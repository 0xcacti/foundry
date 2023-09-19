use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ChaosConfig {}

impl ChaosConfig {
    pub fn new() -> Self {
        Self {}
    }
}

impl FromStr for ChaosConfig {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new())
    }

    // fn from_str(s: &str) -> Result<Self, Self::Err> {
    //     let s = s.to_lowercase();
    //     let hardfork = match s.as_str() {
    //         "frontier" | "1" => Hardfork::Frontier,
    //         "homestead" | "2" => Hardfork::Homestead,
    //         "dao" | "3" => Hardfork::Dao,
    //         "tangerine" | "4" => Hardfork::Tangerine,
    //         "spuriousdragon" | "5" => Hardfork::SpuriousDragon,
    //         "byzantium" | "6" => Hardfork::Byzantium,
    //         "constantinople" | "7" => Hardfork::Constantinople,
    //         "petersburg" | "8" => Hardfork::Petersburg,
    //         "istanbul" | "9" => Hardfork::Istanbul,
    //         "muirglacier" | "10" => Hardfork::Muirglacier,
    //         "berlin" | "11" => Hardfork::Berlin,
    //         "london" | "12" => Hardfork::London,
    //         "arrowglacier" | "13" => Hardfork::ArrowGlacier,
    //         "grayglacier" | "14" => Hardfork::GrayGlacier,
    //         "paris" | "merge" | "15" => Hardfork::Paris,
    //         "shanghai" | "16" => Hardfork::Shanghai,
    //         // "cancun" | "17"=> Hardfork::Cancun,
    //         "latest" => Hardfork::Latest,
    //         _ => return Err(format!("Unknown hardfork {s}")),
    //     };
    //     Ok(hardfork)
    // }
}
