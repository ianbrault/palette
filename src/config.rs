/*
 * src/config.rs
 * author: Ian Brault <ian.brault@engineering.ucla.edu>
 */

use clap::{App, Arg};


pub struct Config {
    pub input_file: String,
    pub output_file: String,
    pub n_colors: u32,
    pub image_output: bool,
    pub term_output: bool,
}

impl Config {
    fn new(args: clap::ArgMatches) -> Config {
        let input_file = String::from(args.value_of("input_file").unwrap());
        let delim = input_file.rfind('.').unwrap();
        let input_base = &input_file[0..delim];
        let file_type = &input_file[(delim+1)..];

        let output_file = if args.is_present("output_file") {
            String::from(args.value_of("output_file").unwrap())
        } else {
            format!("{}_palette.{}", input_base, file_type)
        };

        Config {
            input_file,
            output_file,
            n_colors: args.value_of("n").unwrap_or("5").parse::<u32>().unwrap(),
            image_output: !args.is_present("no_image_output"),
            term_output: args.is_present("term_output"),
        }
    }

    fn valid_n_colors(arg: String) -> Result<(), String> {
        match arg.parse::<u8>() {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn parse() -> Config {
        let version = format!("v{}", env!("CARGO_PKG_VERSION"));
        let args = App::new(env!("CARGO_PKG_NAME"))
            .version(version.as_str())
            .author(env!("CARGO_PKG_AUTHORS"))
            .arg(Arg::with_name("n")
                .short("n")
                .long("n-colors")
                .takes_value(true)
                .validator(Config::valid_n_colors)
                .help("number of palette colors generated (default=5)"))
            .arg(Arg::with_name("output_file")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("output image file"))
            .arg(Arg::with_name("no_image_output")
                .long("no-image")
                .help("skip writing the output image"))
            .arg(Arg::with_name("term_output")
                .short("t")
                .long("term")
                .help("print palette colors to the terminal"))
            .arg(Arg::with_name("input_file")
                .required(true)
                .help("input image file"));

        Config::new(args.get_matches())
    }
}
