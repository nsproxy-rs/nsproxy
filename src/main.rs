use clap;
use std::env;

struct CliArgs {
    target_path: String,
    config_path: String,
}

fn parse_cli() -> CliArgs {
    let program_name = "rproxy";
    let program_version = "0.1.0";
    let program_author = "Not guy";
    let target_arg_name = "target";
    let config_arg_name = "config";
    let default_config_path = env::current_dir().unwrap().as_path().join("rproxy.conf");

    let matches = clap::App::new(program_name)
    .version(program_version)
    .author(program_author)
    .arg(clap::Arg::with_name(target_arg_name)
        .help("target program to run under rproxy")
        .required(true)
        .index(1))
    .arg(clap::Arg::with_name(config_arg_name)
        .help("configuration path")
        .required(false)
        .index(2)
        .default_value(default_config_path.to_str().unwrap()))
    .get_matches();

    CliArgs {
        target_path: String::from(matches.value_of(target_arg_name).unwrap()),
        config_path: String::from(matches.value_of(config_arg_name).unwrap()),
    }
}

fn main() {
    let args = parse_cli();

    println!("Hello, world! target_path: {} config_path: {}", args.target_path, args.config_path);
}
