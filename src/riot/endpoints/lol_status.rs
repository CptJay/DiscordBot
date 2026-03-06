// LOL-STATUS-V4 endpoints stub

pub enum LolStatusEndpoint {
    PlatformData,
}

impl super::RiotEndpoint for LolStatusEndpoint {
    fn path(&self) -> String {
        match self {
            LolStatusEndpoint::PlatformData => "/lol/status/v4/platform-data".to_string(),
        }
    }
}
