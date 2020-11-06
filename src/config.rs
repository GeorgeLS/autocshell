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

#[derive(Debug)]
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
    pub fn from_string(cfg_str: &str) -> Result<Self, String> {
        if cfg_str.is_empty() {
            return Err("Configuration is empty".to_string());
        }

        let mut cfg = Config::default();
        let mut parsing_prog_opt = false;

        let out_of_option_error = |field: &str, line_num: usize| -> Result<Config, String> {
            Err(format!(
                "'{}' field is not allowed outside of an option field (line {})",
                field, line_num
            ))
        };

        let boolean_value_error = |field: &str, line_num: usize| -> Result<Config, String> {
            Err(format!(
                "'{}' accepts only true or false (line {})",
                field, line_num
            ))
        };

        for (line_num, line) in cfg_str.lines().enumerate() {
            let line_num = line_num + 1;
            let mut colon_split = line.split(':');

            if let (Some(field), Some(value)) = (colon_split.next(), colon_split.next()) {
                let field = field.trim();
                let value = value.trim();

                if field.is_empty() {
                    return Err(format!("Missing field in line {}", line_num));
                } else if value.is_empty() && field != "option" {
                    return Err(format!("Missing value in line {}", line_num));
                }

                if field == "shell" {
                    parsing_prog_opt = false;
                    cfg.shell = value.to_string();
                } else if field == "program_name" {
                    parsing_prog_opt = false;
                    cfg.program_name = value.to_string();
                } else if field == "accepts_files" {
                    let value = if value == "true" {
                        true
                    } else if value == "false" {
                        false
                    } else {
                        return boolean_value_error(field, line_num);
                    };

                    if parsing_prog_opt {
                        cfg.program_options.last_mut().unwrap().accepts_files = value;
                    }
                } else if field == "accepts_multiple" {
                    let value = if value == "true" {
                        true
                    } else if value == "false" {
                        false
                    } else {
                        return boolean_value_error(field, line_num);
                    };

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
                        return out_of_option_error(field, line_num);
                    }
                } else if field == "long" {
                    if parsing_prog_opt {
                        cfg.program_options.last_mut().unwrap().long = value.to_string();
                    } else {
                        return out_of_option_error(field, line_num);
                    }
                } else if field == "use_equals_sign" {
                    parsing_prog_opt = false;
                    let value = if value == "true" {
                        true
                    } else if value == "false" {
                        false
                    } else {
                        return boolean_value_error(field, line_num);
                    };

                    cfg.use_equals_sign = value;
                } else if field == "accepts_value" {
                    if parsing_prog_opt {
                        let value = if value == "true" {
                            true
                        } else if value == "false" {
                            false
                        } else {
                            return boolean_value_error(field, line_num);
                        };

                        cfg.program_options.last_mut().unwrap().accepts_value = value;
                    } else {
                        return out_of_option_error(field, line_num);
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
                        return out_of_option_error(field, line_num);
                    }
                } else if field == "option" {
                    parsing_prog_opt = true;
                    cfg.program_options.push(ProgramOption::default());
                } else {
                    return Err(format!("Unknown field '{}' in line {}", field, line_num));
                }
            }
        }

        if cfg.program_name.is_empty() {
            return Err("Configuration is missing mandatory field 'program_name'".to_string());
        }

        if cfg.program_options.is_empty() {
            return Err("Configuration has no program options defined".to_string());
        }

        Ok(cfg)
    }

    pub fn from_file(cfg_filename: &str) -> Result<Self, String> {
        let cfg_contents = fs::read_to_string(cfg_filename)
            .or_else(|_| Err("Couldn't read configuration file."))?;

        Ok(Config::from_string(&cfg_contents)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_config_should_fail() {
        let cfg_str = "";
        let cfg = Config::from_string(cfg_str);
        assert!(cfg.is_err());
        assert_eq!(cfg.unwrap_err(), "Configuration is empty");
    }

    #[test]
    fn config_with_unknown_field_should_fail() {
        let cfg_str = "\
            shell: bash
            program_name: test
            foo: foo_value
        ";

        let cfg = Config::from_string(cfg_str);
        assert!(cfg.is_err());
        assert_eq!(cfg.unwrap_err(), "Unknown field 'foo' in line 3")
    }

    #[test]
    fn config_with_missing_field_should_fail() {
        let cfg_str = "\
            shell: bash
            : value
        ";

        let cfg = Config::from_string(cfg_str);
        assert!(cfg.is_err());
        assert_eq!(cfg.unwrap_err(), "Missing field in line 2");
    }

    #[test]
    fn config_with_missing_value_should_fail() {
        let cfg_str = "\
            shell:
            program_name: _invalid_
        ";

        let cfg = Config::from_string(cfg_str);
        assert!(cfg.is_err());
        assert_eq!(cfg.unwrap_err(), "Missing value in line 1");
    }

    #[test]
    fn config_without_options_shoud_fail() {
        let cfg_str = "\
            shell: zsh
            program_name: prog
        ";

        let cfg = Config::from_string(cfg_str);
        assert!(cfg.is_err());
        assert_eq!(
            cfg.unwrap_err(),
            "Configuration has no program options defined"
        );
    }

    #[test]
    fn config_with_missing_program_name_should_fail() {
        let cfg_str = "\
            shell: zsh
            option:
                short: -h
                long: --help
        ";

        let cfg = Config::from_string(cfg_str);
        assert!(cfg.is_err());
        assert_eq!(
            cfg.unwrap_err(),
            "Configuration is missing mandatory field 'program_name'"
        );
    }

    #[test]
    fn option_field_not_allowed_outside_of_option() {
        let cfg_str = "\
            shell: bash
            program_name: test
            short: -v
            option:
                short: -v
                long: --version
        ";

        let cfg = Config::from_string(cfg_str);
        assert!(cfg.is_err());
        assert_eq!(
            cfg.unwrap_err(),
            "'short' field is not allowed outside of an option field (line 3)"
        );
    }

    #[test]
    fn unknown_field_inside_option_should_fail() {
        let cfg_str = "\
            shell: zsh
            program_name: test
            option:
                invalid: -s
                long: --crash
        ";

        let cfg = Config::from_string(cfg_str);
        assert!(cfg.is_err());
        assert_eq!(cfg.unwrap_err(), "Unknown field 'invalid' in line 4");
    }

    #[test]
    fn boolean_fields_should_accept_only_true_or_false() {
        let cfg_str = "\
            program_name: test
            use_equals_sign: true
            option:
                short: -h
                accepts_value: blah
        ";

        let cfg = Config::from_string(cfg_str);
        assert!(cfg.is_err());
        assert_eq!(
            cfg.unwrap_err(),
            "'accepts_value' accepts only true or false (line 5)"
        );
    }

    #[test]
    fn missing_fields_are_set_to_default() {
        let cfg_str = "\
            shell: bash
            program_name: test
            option:
                short: -v
                long: --version
                description: Some description
        ";

        let cfg = Config::from_string(cfg_str);
        assert!(cfg.is_ok());
        let cfg = cfg.unwrap();
        assert_eq!(cfg.use_equals_sign, true);
        assert_eq!(cfg.program_options.len(), 1);
        let prog_opt = &cfg.program_options[0];
        assert_eq!(prog_opt.accepts_value, true);
        assert_eq!(prog_opt.accepts_files, false);
        assert_eq!(prog_opt.accepts_multiple, false);
    }

    #[test]
    fn missing_fields_are_set_to_default_extra() {
        let cfg_str = "\
            program_name: test
            option:
                long: --help
            option:
                short: -h
                accepts_value: false
        ";

        let cfg = Config::from_string(cfg_str);
        assert!(cfg.is_ok());

        let cfg = cfg.unwrap();
        assert_eq!(cfg.use_equals_sign, true);
        assert_eq!(cfg.program_options.len(), 2);

        let opt_0 = &cfg.program_options[0];
        assert_eq!(opt_0.accepts_value, true);
        assert_eq!(opt_0.accepts_files, false);
        assert_eq!(opt_0.accepts_multiple, false);
        assert_eq!(opt_0.description, "");
        assert_eq!(opt_0.short, "");
        assert_eq!(opt_0.fixed_values, Vec::<String>::new());

        let opt_1 = &cfg.program_options[1];
        assert_eq!(opt_1.accepts_value, false);
        assert_eq!(opt_1.accepts_files, false);
        assert_eq!(opt_1.accepts_multiple, false);
        assert_eq!(opt_1.description, "");
        assert_eq!(opt_1.long, "");
        assert_eq!(opt_1.fixed_values, Vec::<String>::new());
    }

    #[test]
    fn config_parses_correctly_1() {
        let cfg_str = "\
            program_name: test_2
            use_equals_sign: false
            option:
                short: -v
                long: --version
                description: Display the program version
                accepts_value: false
            option:
                long: --help
                description: Display helpful information
                accepts_value: false
            option:
                short: -o
                long: --output
                description: Specify the output path
                fixed_values: [log_file]
        ";

        let cfg = Config::from_string(cfg_str);
        assert!(cfg.is_ok());

        let cfg = cfg.unwrap();
        assert_eq!(cfg.shell, "");
        assert_eq!(cfg.program_name, "test_2");
        assert_eq!(cfg.program_options.len(), 3);
        assert_eq!(cfg.use_equals_sign, false);

        let opt_0 = &cfg.program_options[0];
        assert_eq!(opt_0.accepts_files, false);
        assert_eq!(opt_0.accepts_value, false);
        assert_eq!(opt_0.accepts_multiple, false);
        assert_eq!(opt_0.description, "Display the program version");
        assert_eq!(opt_0.short, "-v");
        assert_eq!(opt_0.long, "--version");
        assert_eq!(opt_0.fixed_values, Vec::<String>::new());

        let opt_1 = &cfg.program_options[1];
        assert_eq!(opt_1.accepts_files, false);
        assert_eq!(opt_1.accepts_value, false);
        assert_eq!(opt_1.accepts_multiple, false);
        assert_eq!(opt_1.description, "Display helpful information");
        assert_eq!(opt_1.short, "");
        assert_eq!(opt_1.long, "--help");
        assert_eq!(opt_1.fixed_values, Vec::<String>::new());

        let opt_2 = &cfg.program_options[2];
        assert_eq!(opt_2.accepts_files, false);
        assert_eq!(opt_2.accepts_value, true);
        assert_eq!(opt_2.accepts_multiple, false);
        assert_eq!(opt_2.description, "Specify the output path");
        assert_eq!(opt_2.short, "-o");
        assert_eq!(opt_2.long, "--output");
        assert_eq!(opt_2.fixed_values, vec!["log_file"]);
    }
}
