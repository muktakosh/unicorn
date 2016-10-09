extern crate unicorn;
extern crate clap;

#[macro_use]
extern crate log;

use unicorn::logger::CLILogger;
use std::thread;
use std::error::Error;
use clap::{App, Arg, SubCommand, AppSettings};

fn run_all() {
    info!("Running all components from configuration");
}

fn init_logger(loglevel: &str) { CLILogger::init(loglevel).unwrap() }

fn main() {

    let v: String = unicorn::get_version();
    let matches = App::new("unicorn")
        .settings(&[AppSettings::ArgRequiredElseHelp,
                   AppSettings::VersionlessSubcommands])
        .version(v.as_str())
        .author("The Muktakosh Project Developers")
        .about("Unified Communications Over Real-time Networks")
        .after_help("For more information on unicorn, see: http://labs.muktakosh.org.")

        // Flags
        .arg(Arg::with_name("debug")
             .help("Show verbose output. Sets log level to `debug`")
             .short("d")
             .long("debug")
             .global(true))

        // Options
        .arg(Arg::with_name("config")
             .help("Provide a configuration file")
             .short("c")
             .long("config")
             .takes_value(true)
             .value_name("FILE"))

        // Subcommand: `run`
        .subcommand(SubCommand::with_name("run")
                    .about("Run a unicorn component or plugin")
                    .arg(Arg::with_name("component")
                         .help("Name of a component or plugin to run")
                         .index(1)
                         .default_value("all")
                         .possible_values(&["api", "all", "datastore"])))

        // Match them up
        .get_matches();

    // Set log level and start logger
    if matches.is_present("debug") {
        init_logger("debug");
    } else {
        init_logger("info");
    }

    // Load configuration
    let mut configpath = "unicorn.json";

    if matches.is_present("config") {
        if let Some(ref c) = matches.value_of("config") {
            configpath = c;
        }
    }

    match unicorn::config::load(configpath) {
        Ok(s) => debug!("Config:\n\t{:?}", s),
        Err(e) => error!("Unable to load config file: {}. Error: {}", configpath, e.description())
    }

    // Parse the `run` subcommand
    if let Some(ref run) = matches.subcommand_matches("run") {
        if run.is_present("component") {
            match run.value_of("component") {
                Some("api") => unicorn::kernel::run(),
                Some("all") => run_all(),
                Some("datastore") => unimplemented!(),
                Some(_) | None => println!("No components matched. See `unicorn help run` for available options.")
            }
        }
    }

}
