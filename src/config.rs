use std::{default::Default, fs};

pub struct ProgramOption {
    pub short: String,
    pub long: String,
    pub accepts_files: bool,
}

impl Default for ProgramOption {
    fn default() -> Self {
        Self {
            short: String::new(),
            long: String::new(),
            accepts_files: false,
        }
    }
}

pub struct Config {
    pub shell_type: String,
    pub prog_name: String,
    pub prog_options: Vec<ProgramOption>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            shell_type: String::new(),
            prog_name: String::new(),
            prog_options: Vec::new(),
        }
    }
}

impl Config {
    pub fn from_file(cfg_filename: &str) -> Result<Self, String> {
        let mut cli = Config::default();
        let cfg_contents = fs::read_to_string(cfg_filename)
            .or_else(|_| Err("Couldn't read configuration file."))?;

        let mut parsing_prog_opt = false;
        for (line_num, line) in cfg_contents.lines().enumerate() {
            let line_num = line_num + 1;
            let mut colon_split = line.split(':');

            if let (Some(field), Some(value)) = (colon_split.next(), colon_split.next()) {
                let field = field.trim();
                let value = value.trim();

                if field.is_empty() {
                    return Err(std::format!("Missing field name in line {}.", line_num));
                } else if value.is_empty() && field != "option" {
                    return Err(std::format!("Missing value in line {}.", line_num));
                }

                if field == "shell" {
                    parsing_prog_opt = false;
                    cli.shell_type = value.to_string();
                } else if field == "program_name" {
                    parsing_prog_opt = false;
                    cli.prog_name = value.to_string();
                } else if field == "accepts_files" {
                    let value = value == "true";
                    if parsing_prog_opt {
                        cli.prog_options.last_mut().unwrap().accepts_files = value;
                    }
                } else if field == "short" {
                    if parsing_prog_opt {
                        cli.prog_options.last_mut().unwrap().short = value.to_string();
                    } else {
                        return Err(
                            "Short field is not allowed outside of an option field".to_string()
                        );
                    }
                } else if field == "long" {
                    if parsing_prog_opt {
                        cli.prog_options.last_mut().unwrap().long = value.to_string();
                    } else {
                        return Err(
                            "Long field is not allowed outside of an option field".to_string()
                        );
                    }
                } else if field == "option" {
                    parsing_prog_opt = true;
                    cli.prog_options.push(ProgramOption::default());
                } else {
                    return Err(std::format!("Unknown field `{}`.", field));
                }
            }
        }

        Ok(cli)
    }
}
