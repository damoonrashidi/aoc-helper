use std::{fs::File, io::Read, path::PathBuf};
use toml::Value;

#[derive(serde::Deserialize)]
pub(crate) struct AdventConfig {
    pub(crate) project: ProjectConfig,
    pub(crate) session: SessionConfig,
}

#[derive(serde::Deserialize)]
pub(crate) struct ProjectConfig {
    pub(crate) code_directory: String,
    pub(crate) input_directory: String,
}

#[derive(serde::Deserialize)]
pub(crate) struct SessionConfig {
    pub(crate) secret: String,
}

impl AdventConfig {
    pub(crate) fn get() -> Self {
        let config_str = simple_home_dir::home_dir()
            .map(|dir| dir.join(".aoc_config"))
            .expect("couldn't get session file");

        AdventConfig::from_file(config_str)
    }

    pub(crate) fn from_file(path: PathBuf) -> AdventConfig {
        let mut file = File::open(path).expect("Could not find config file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Could not read file")
            .to_string();

        let value: Value = toml::from_str(&contents).expect("Could not parse");

        let config: AdventConfig = value.try_into().expect("could not parse");

        config
    }
}
