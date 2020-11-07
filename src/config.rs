use std::{
    default::Default,
    fs,
    iter::{Enumerate, Peekable},
    str::Lines,
};

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
    pub fn has_one_representation(&self) -> bool {
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

#[inline]
fn check_bool(value: &str) -> Option<bool> {
    if value == "true" {
        Some(true)
    } else if value == "false" {
        Some(false)
    } else {
        None
    }
}

#[inline]
fn boolean_value_error(field: &str, line_num: usize) -> String {
    format!("'{}' accepts only true or false (line {})", field, line_num)
}

type FieldValueResult<'l> = Result<Option<(&'l str, &'l str, usize)>, String>;

fn next_field_and_value_base(line: &str, line_num: usize) -> FieldValueResult<'_> {
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

        Ok(Some((field, value, line_num)))
    } else {
        Ok(None)
    }
}

fn peek_field_and_value<'l>(
    line_it: &'l mut Peekable<Enumerate<Lines>>,
) -> FieldValueResult<'l> {
    if let Some((line_num, line)) = line_it.peek() {
        next_field_and_value_base(line, *line_num)
    } else {
        Ok(None)
    }
}

fn next_field_and_value<'l>(
    line_it: &'l mut Peekable<Enumerate<Lines>>
) -> FieldValueResult<'l> {
    if let Some((line_num, line)) = line_it.next() {
        next_field_and_value_base(line, line_num)
    } else {
        Ok(None)
    }
}

impl Config {
    pub fn from_file(cfg_filename: &str) -> Result<Self, String> {
        let cfg_contents = fs::read_to_string(cfg_filename)
            .or_else(|_| Err("Couldn't read configuration file.".to_string()))?;

        let cfg = Config::from_string(&cfg_contents)?;
        Ok(cfg)
    }

    pub fn from_string(cfg_str: &str) -> Result<Self, String> {
        if cfg_str.is_empty() {
            return Err("Configuration is empty".to_string());
        }

        let mut cfg = Config::default();
        let mut line_it = cfg_str.lines().enumerate().peekable();

        while let Some((field, value, line_num)) = next_field_and_value(&mut line_it)? {
            match field {
                "shell" => cfg.shell = value.to_owned(),
                "program_name" => cfg.program_name = value.to_owned(),
                "use_equals_sign" => {
                    cfg.use_equals_sign =
                        check_bool(value).ok_or(boolean_value_error(field, line_num))?;
                }
                "option" => {
                    let program_option = Config::parse_program_option(&mut line_it)?;
                    cfg.program_options.push(program_option);
                }
                _ => {
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

    fn parse_program_option(
        line_it: &mut Peekable<Enumerate<Lines>>,
    ) -> Result<ProgramOption, String> {
        let mut program_option = ProgramOption::default();
        while let Some((field, value, line_num)) = peek_field_and_value(line_it)? {
            if field == "option" {
                break;
            }

            match field {
                "short" => program_option.short = value.to_owned(),
                "long" => program_option.long = value.to_owned(),
                "description" => program_option.description = value.replace("'", "\\'"),
                "accepts_files" => {
                    program_option.accepts_files =
                        check_bool(value).ok_or(boolean_value_error(field, line_num))?
                }
                "accepts_multiple" => {
                    program_option.accepts_multiple =
                        check_bool(value).ok_or(boolean_value_error(field, line_num))?
                }
                "accepts_value" => {
                    program_option.accepts_value =
                        check_bool(value).ok_or(boolean_value_error(field, line_num))?
                }
                "fixed_values" => {
                    let fixed_values = Config::parse_fixed_values(value, line_num)?;
                    program_option.fixed_values = fixed_values;
                }
                _ => return Err(format!("Unknown field '{}' in line {}", field, line_num)),
            }

            line_it.next();
        }

        Ok(program_option)
    }

    fn parse_fixed_values(fixed_values: &str, line_num: usize) -> Result<Vec<String>, String> {
        if let (Some(start), Some(end)) = (fixed_values.find('['), fixed_values.find(']')) {
            let fixed_values = &fixed_values[(start + 1)..end];
            let fixed_values: Vec<_> = fixed_values
                .split(',')
                .map(|v| v.trim().replace("'", "\\'"))
                .collect();

            Ok(fixed_values)
        } else {
            Err(format!(
                "'fixed_values' has incorrect format. Expected [<fixed_value>, ...] (line {})",
                line_num
            ))
        }
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
    fn config_without_options_should_fail() {
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
            "Unknown field 'short' in line 3"
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

    #[test]
    fn description_and_fixed_values_should_be_escaped() {
        let cfg_str = "\
            program_name: test
            option:
                short: -q
                long: --quiet
                description: Don't display output
            option:
                long: --value
                fixed_values: [don't, it's]
        ";

        let cfg = Config::from_string(cfg_str);
        assert!(cfg.is_ok());
        let cfg = cfg.unwrap();

        assert_eq!(cfg.program_options[0].description, "Don\\'t display output");
        assert_eq!(cfg.program_options[1].fixed_values, vec!["don\\'t", "it\\'s"]);
    }
}
