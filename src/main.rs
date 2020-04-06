use clap;
use std::env;

struct CliArgs {
    target_path: String,
    config_path: String,
}

fn parse_cli() -> CliArgs {
    let target_arg_name = "target";
    let config_arg_name = "config";
    let default_config_path = env::current_dir().unwrap().as_path().join("nsproxy.conf");

    let matches = clap::App::new("nsproxy")
    .version("0.1.0")
    .author("Guy Bortnikov, Liran Ringel, Shay Sandler")
    .arg(clap::Arg::with_name(target_arg_name)
        .help("target program (binary) to run under nsproxy")
        .required(true)
        .index(1))
    .arg(clap::Arg::with_name(config_arg_name)
        .help("configuration path, default will be taken 'nsproxy.conf' from the current directory")
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
