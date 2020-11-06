use clap::{App, Arg};

pub struct Cli {
    pub shell: Option<String>,
    pub cfg_file: String,
    pub output_file: Option<String>,
    pub show_cfg_help: bool,
}

fn build_app<'a, 'b>() -> App<'a, 'b> {
    App::new("autocshell")
        .version("0.5.2")
        .author("George Liontos <georgeliontos98@gmail.com>")
        .about("Generate autocompletion shell scripts for you application!")
        .arg(
            Arg::with_name("cfg_file")
                .short("c")
                .long("config")
                .value_name("CONFIG_FILE")
                .help("Specify the configuration filename to read the autocomplete specification from")
                .takes_value(true)
                .min_values(1)
                .max_values(1)
                .required_unless("cfg_help")
        )
        .arg(
            Arg::with_name("output_file")
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
        .arg(
            Arg::with_name("shell")
                .short("s")
                .long("shell")
                .takes_value(true)
                .min_values(1)
                .max_values(1)
                .help("Specify the shell to generate the script for")
        )
}

impl Cli {
    pub fn from_args() -> Self {
        let app = build_app();
        let matches = app.get_matches();

        let show_cfg_help = matches.is_present("cfg_help");
        let shell = matches.value_of("shell").map(|s| s.to_owned());
        let output_file = matches.value_of("output_file").map(|o| o.to_owned());
        let cfg_file = matches
            .value_of("cfg_file")
            .expect("Please provide a configuratio file")
            .to_owned();

        Self {
            shell,
            cfg_file,
            output_file,
            show_cfg_help,
        }
    }
}
