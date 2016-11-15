extern crate unicorn;
extern crate clap;

#[macro_use]
extern crate log;

use unicorn::logger::CLILogger;
use std::thread;
use std::error::Error;
use clap::{App, Arg, SubCommand, AppSettings};

fn run_all(conf: unicorn::schema::config_schema::Config) {
    info!("Running all components from configuration");
    let c1 = conf.clone();
    let kernel = thread::spawn(move || {
        unicorn::kernel::run(c1);
    });
    let _ = kernel.join();
}

fn init_logger(loglevel: &str) {
    CLILogger::init(loglevel).unwrap()
}

fn main() {

    let v: String = unicorn::get_version();
    let matches = App::new("unicorn")
        .settings(&[AppSettings::ArgRequiredElseHelp,
                    AppSettings::VersionlessSubcommands,
                    AppSettings::SubcommandRequiredElseHelp
        ])
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

        // Subcommand: `init`
        .subcommand(SubCommand::with_name("init")
                    .about("Initialize a unicorn config in current directory"))

        // Match them up
        .get_matches();

    // Set log level and start logger
    if matches.is_present("debug") {
        init_logger("debug");
    } else {
        init_logger("info");
    }

    // Init configuration: `init` subcommand
    if matches.subcommand_matches("init").is_some() {
        match unicorn::config::init() {
            Ok(()) => {
                info!("Created config file in ./unicorn.json");
                std::process::exit(0);
            }
            Err(e) => {
                error!("Unable to create config file: {}", e.description());
                std::process::exit(1);
            }
        }

    }

    // Load configuration
    let mut configpath = "unicorn.json";

    if matches.is_present("config") {
        if let Some(c) = matches.value_of("config") {
            configpath = c;
        }
    }

    let conf = match unicorn::config::load(configpath) {
        Ok(c) => {
            debug!("Config:\n\t{:?}", c);
            c
        }
        Err(e) => {
            error!("Invalid config file: {}. Error: {}",
                   configpath,
                   e.description());
            error!("Using default config");
            unicorn::config::default()
        }
    };

    // Parse the `run` subcommand
    if let Some(run) = matches.subcommand_matches("run") {
        if run.is_present("component") {
            match run.value_of("component") {
                Some("api") => unicorn::kernel::run(conf),
                Some("all") => run_all(conf),
                Some("datastore") => unimplemented!(),
                Some(_) | None => {
                    println!("No components matched. See `unicorn help run` for available options.")
                }
            }
        }
    }

}
