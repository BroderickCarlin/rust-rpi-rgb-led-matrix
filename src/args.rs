use crate::c::LedMatrixOptions;
use clap;

pub fn add_matrix_args(&mut clap::App: app) {
    app.arg(
        Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .about("Sets a custom config file")
            .takes_value(true),
    );
}

pub fn matrix_options_from_args(&clap::ArgMatches: parsed_args<'a>) -> LedMatrixOptions {
    let mut options = LedMatrixOptions::new();
}
