use std::{default::Default, fs};

#[derive(Debug, Clone)]
pub struct ProgramOption {
    pub short: String,
    pub long: String,
    pub description: String,
    pub fixed_values: Vec<String>,
    pub accepts_files: bool,
    pub accepts_multiple: bool,
    pub accepts_value: bool,
}

impl Default for ProgramOption {
    fn default() -> Self {
        Self {
            short: String::new(),
            long: String::new(),
            description: String::new(),
            fixed_values: Vec::new(),
            accepts_files: false,
            accepts_multiple: false,
            accepts_value: true,
        }
    }
}

impl ProgramOption {
    #[inline(always)]
    pub fn has_one_represenation(&self) -> bool {
        self.short.is_empty() ^ self.long.is_empty()
    }

    #[inline(always)]
    pub fn is_help(&self) -> bool {
        self.short == "-h" || self.long == "--help"
    }
}

pub struct Config {
    pub shell: String,
    pub program_name: String,
    pub program_options: Vec<ProgramOption>,
    pub use_equals_sign: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            shell: String::new(),
            program_name: String::new(),
            program_options: Vec::new(),
            use_equals_sign: true,
        }
    }
}

impl Config {
    pub fn from_file(cfg_filename: &str) -> Result<Self, String> {
        let mut cfg = Config::default();
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
                    cfg.shell = value.to_string();
                } else if field == "program_name" {
                    parsing_prog_opt = false;
                    cfg.program_name = value.to_string();
                } else if field == "accepts_files" {
                    let value = value == "true";
                    if parsing_prog_opt {
                        cfg.program_options.last_mut().unwrap().accepts_files = value;
                    }
                } else if field == "accepts_multiple" {
                    let value = value == "true";
                    if parsing_prog_opt {
                        cfg.program_options.last_mut().unwrap().accepts_multiple = value;
                    }
                } else if field == "description" {
                    if parsing_prog_opt {
                        cfg.program_options.last_mut().unwrap().description = value.to_string();
                    }
                } else if field == "short" {
                    if parsing_prog_opt {
                        cfg.program_options.last_mut().unwrap().short = value.to_string();
                    } else {
                        return Err(
                            "Short field is not allowed outside of an option field".to_string()
                        );
                    }
                } else if field == "long" {
                    if parsing_prog_opt {
                        cfg.program_options.last_mut().unwrap().long = value.to_string();
                    } else {
                        return Err(
                            "Long field is not allowed outside of an option field".to_string()
                        );
                    }
                } else if field == "use_equals_sign" {
                    parsing_prog_opt = false;
                    let value = value == "true";
                    cfg.use_equals_sign = value;
                } else if field == "accepts_value" {
                    if parsing_prog_opt {
                        let value = value == "true";
                        cfg.program_options.last_mut().unwrap().accepts_value = value;
                    } else {
                        return Err(
                            "accepts_value field is not allowed outside of an option field"
                                .to_string(),
                        );
                    }
                } else if field == "fixed_values" {
                    if parsing_prog_opt {
                        if let (Some(list_start_index), Some(list_end_index)) =
                            (value.find('['), value.find(']'))
                        {
                            let fixed_values = &value[(list_start_index + 1)..list_end_index];
                            let fixed_values: Vec<_> = fixed_values
                                .split(',')
                                .map(|v| v.trim().to_string())
                                .collect();
                            cfg.program_options.last_mut().unwrap().fixed_values = fixed_values;
                        } else {
                            return Err("fixed_values list value is in incorrect format. Expected [<fixed_value>, ...]".to_string());
                        }
                    } else {
                        return Err(
                            "fixed_values field is not allowed outside of an option field"
                                .to_string(),
                        );
                    }
                } else if field == "option" {
                    parsing_prog_opt = true;
                    cfg.program_options.push(ProgramOption::default());
                } else {
                    return Err(std::format!("Unknown field `{}`.", field));
                }
            }
        }

        Ok(cfg)
    }
}
