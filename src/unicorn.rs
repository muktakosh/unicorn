extern crate unicorn;
extern crate clap;

#[macro_use]
extern crate log;

use unicorn::logger::CLILogger;
use std::thread;
use std::error::Error;
use clap::{App, Arg, SubCommand, AppSettings};

fn run_as_single_process() {
    info!("Running as single process");
    let core_th = thread::spawn(move || {
        unicorn::kernel::run();
    });
    core_th.join().unwrap();
}

fn run_all() {
    info!("Running all components from configuration");
}

fn show_components_list() {
    info!("List of components available:");
    info!("\tkernel");
    info!("\tlite");
    info!("\tall");
    info!("\nUse `unicorn run <component>` to run them. For details, see `unicorn help run`.");
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
                         .index(1))
                    .arg(Arg::with_name("list-components")
                         .help("List installed components that can be run")
                         .short("l")
                         .long("list-components")
                         .conflicts_with_all(&["component"])))

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

        if run.is_present("list-components") {
            show_components_list();
        }

        if run.is_present("component") {
            match run.value_of("component") {
                Some("kernel") => unicorn::kernel::run(),
                Some("lite") => run_as_single_process(),
                Some("all") | None => run_all(),
                Some(_) => println!("No components matched. See `unicorn run --list-components` for available options.")
            }
        }
    }

}
