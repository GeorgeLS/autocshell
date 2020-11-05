mod bash;
mod cli;
mod config;
mod zsh;

use cli::Cli;
use config::*;
use std::{error::Error, fs, io, process::exit};

fn show_cfg_help_and_exit() -> ! {
    println!(
        "
The configuration file that you must provide as input (using -c or --config option)
has the following format:

shell:        <shell_type> (bash|zsh)
program_name: <program_name>
use_equals_sign: (true|false) [default: true] (available only for zsh)
option*:
    short?: <short_name> _
                          |-> At least one should exist
    long?:  <long_name>  ‾
    accepts_value?:    (true|false) [default: true]  (available only for zsh)
    accepts_files?:    (true|false) [default: false]
    accepts_multiple?: (true|false) [default: false] (available only for zsh)
    description?:
    fixed_values?:     [<fixed_value>, ...]

Field/Values explanation:

Field: shell
Value: It's the shell you want to generate the script for. Currently supported: bash
Mandatory: yes

Field: program_name
Value: The name of you program to generate the autocompletions for
Mandatory: yes

Field: use_equals_sign
Value: Denotes whether we want to add an equals sign (=) after option completion
       This is valid only for zsh.
Default: true
Mandatory: no

Field: option
Value: None. The option field gets no value. It starts a new option description
Mandatory: no

Field: short
Value: The short option description (- must be included)
Mandatory: no*

Field: long
Value: The long option description (-- must be included)
Mandatory: no*

Field: accepts_value
Value: Denotes whether this option takes an option or not (it's a flag)
       This is valid only for zsh.
Default: true
Mandatory: no

Field: accepts_files
Value: Denotes whether that option takes files/directories as value(s). Must be true or false
Default: false
Mandatory: no

Field: accepts_multiple
Value: This value denotes whether the option can appear multiple times in the cli
       or take multiple values (which is the same thing). This is valid only for zsh (for now).
       It is ignored for other shells if provided.
Default: false
Mandatory: no

Field: description
Value: This value contains the description that will appear when auto completing this option.
       This is valid only for zsh.
       It is ignored for other shells if provided.
Mandatory: no

Field: fixed_values
Value: This value is a bracketed comma separated list of fixed values that will be auto completed for that option
       This is valid only for zsh.
Mandatory: no

* short and long fields are not mandatory, however if you define an option at least one of them must be present."
    );

    exit(0);
}

fn write_script_to_file(script: &str, filename: &str, cfg: &Config) -> io::Result<()> {
    let output_file = if filename.contains(&cfg.shell) {
        filename.to_owned()
    } else {
        std::format!("{}.{}", filename, cfg.shell)
    };

    fs::write(output_file, script)?;
    Ok(())
}

fn generate_script(cfg: &Config) -> Option<String> {
    if cfg.shell == "bash" {
        Some(bash::generate_bash(cfg))
    } else if cfg.shell == "zsh" {
        Some(zsh::generate_zsh(cfg))
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::from_args();

    if cli.show_cfg_help {
        show_cfg_help_and_exit();
    }

    let mut cfg = Config::from_file(&cli.cfg_file)?;
    if cfg.shell.is_empty() {
        if let Some(shell) = cli.shell {
            cfg.shell = shell;
        } else {
            eprintln!("You must provide a shell either from command line or in config file");
            exit(-1);
        }
    }

    let script = generate_script(&cfg);

    match script {
        Some(script) => match cli.output_file {
            Some(output_file) => {
                write_script_to_file(&script, &output_file, &cfg)?;
            }
            None => {
                println!("{}", script);
            }
        },
        None => eprintln!("Shell `{}` is not supported", cfg.shell),
    }

    Ok(())
}
