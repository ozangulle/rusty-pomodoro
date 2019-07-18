use std::collections::BTreeMap;
use std::error::Error;

pub struct YamlConfig<'a> {
    file_name: &'a str,
    config_values: BTreeMap<String, String>,
}

impl<'a> YamlConfig<'a> {
    pub fn new(file_name: &str) -> YamlConfig {
        YamlConfig {
            file_name,
            config_values: BTreeMap::new(),
        }
    }

    pub fn parse(&mut self) -> Result<(), Box<Error>> {
        let file = std::fs::File::open(self.file_name)?;
        self.config_values = serde_yaml::from_reader(file)?;
        Ok(())
    }

    pub fn record_filename(&self) -> Option<&String> {
        self.config_values.get(&"record_name".to_string())
    }

    pub fn record_location(&self) -> Option<&String> {
        self.config_values.get(&"record_location".to_string())
    }
}
