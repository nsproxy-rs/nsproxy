extern crate clap;
// use clap::[App, Arg];

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
        .index(2))
    .get_matches();

    CliArgs {
        target_path: String::from(matches.value_of(target_arg_name).unwrap()),
        config_path: String::from(matches.value_of(config_arg_name).unwrap_or("/etc/rproxy/rproxy.conf"))
    }
}

fn main() {
    let args = parse_cli();

    println!("Hello, world! target_path: {} config_path: {}", args.target_path, args.config_path);
}
