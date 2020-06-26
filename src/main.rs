use clap::{App, Arg};

mod config;
mod generate;

use config::*;
use generate::*;
use std::{
    error::Error,
    fs::File,
    io::{self, prelude::*},
};

fn print_config_file_help() {
    println!(
        "
The configuration file that you must provide as input (using -c or --config option)
has the following format:

shell:        <shell_type> (bash|zsh)
program_name: <program_name>
option*:
short?: <short_name> _
                      |-> At least one should exist
long?:  <long_name>  ‾
accepts_files?:       (true|false) [default: false]

Field/Values explanation:

Field: shell
Value: It's the shell you want to generate the script for. Currently supported: bash
Mandatory: yes

Field: program_name
Value: The name of you program to generate the autocompletions for
Mandatory: yes

Field: option
Value: None. The option field gets no value. It starts a new option description
Mandatory: no

Field: short
Value: The short option description (- must be included)
Mandatory: no*

Field: long
Value: The long option description (-- must be included)
Mandatory: no*

Field: accepts_fiels
Value: Denotes whether that option takes files/directories as value(s). Must be true or false
Default: false

* short and long fields are not mandatory, however if you define an option at least one of them must be present."
    );
}

fn write_script_to_file(script: String, filename: String, cfg: &Config) -> io::Result<()> {
    let out_filename = if filename.contains(cfg.shell_type.as_str()) {
        filename
    } else {
        std::format!("{}.{}", filename, cfg.shell_type)
    };

    let mut out_file = File::create(out_filename)?;
    out_file.write(script.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("autocshell")
        .version("1.0")
        .author("George Liontos <georgeliontos98@gmail.com>")
        .about("Generate autocompletion shell scripts for you application!")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("CONFIG_FILE")
                .help("Specify the configuration filename to read the autocomplete specification from.")
                .takes_value(true)
                .min_values(1)
                .max_values(1)
                .required(true)
                .required_unless("cfg_help")
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT_FILE")
                .help("Specify the name of the output file. The shell extension is appened automatically (e.g <out_name>.bash)")
                .takes_value(true)
                .min_values(1)
                .max_values(1)
        )
        .arg(
            Arg::with_name("cfg_help")
                .long("config-help")
                .takes_value(false)
                .help("Show help/explanation about the configuration file")
        )
        .get_matches();

    if matches.is_present("cfg_help") {
        print_config_file_help();
    } else {
        let cfg_filename = matches
            .value_of("config")
            .ok_or("You must provide a configuration file")?;
        let cfg = Config::from_file(cfg_filename)?;
        let script = generate_script(&cfg);

        if matches.is_present("output") {
            let out_filename = matches.value_of("output").unwrap().to_string();
            write_script_to_file(script, out_filename, &cfg)?;
        } else {
            println!("{}", script);
        }
    }

    Ok(())
}
