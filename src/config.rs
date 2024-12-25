use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct GenericServerConfiguration {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct GeneralConfiguration {
    pub database_timeout: u64,
}

impl GenericServerConfiguration {
    pub fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Deserialize)]
pub struct Configuration {
    pub general: GeneralConfiguration,
    pub server: GenericServerConfiguration,
    pub metrics: GenericServerConfiguration,
}

pub fn load_configuration(path: &str) -> Configuration {
    let file_content = std::fs::read_to_string(path).expect("Unable to read configuration file");
    toml::from_str(&file_content).expect("Unable to parse configuration file")
}
