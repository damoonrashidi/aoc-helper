use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use crate::config::AdventConfig;

pub(crate) struct Creator {
    config: AdventConfig,
}

impl Creator {
    pub(crate) fn new(config: AdventConfig) -> Self {
        Self { config }
    }

    pub(crate) fn code_file(
        &self,
        year: &String,
        day: &String,
        test_input: &str,
        test_answer: &str,
    ) -> Result<usize, std::io::Error> {
        let template = include_str!("../template.txt")
            .replace("{year}", year)
            .replace("{day}", day)
            .replace("{test_input}", test_input)
            .replace("{test_answer}", test_answer);

        let path =
            PathBuf::from(&self.config.project.code_directory).join(format!("{year}_{day}.rs"));
        let mut file = File::create(path).expect("could not create file");

        file.write(template.as_bytes())
    }

    #[allow(unused)]
    pub(crate) fn input_file(
        &self,
        year: &String,
        day: &String,
        input: &String,
    ) -> Result<usize, std::io::Error> {
        let input_folder = PathBuf::from(&self.config.project.input_directory).join(year);
        let _ = create_dir_all(&input_folder);
        let location = input_folder.join(format!("{day}.txt"));
        let mut file = File::create(location).expect("could not create file");

        file.write(input.as_bytes())
    }
}
