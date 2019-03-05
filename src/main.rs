extern crate notify;
extern crate log;
extern crate simple_logger;
extern crate clap;

use log::{debug, error};

use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use clap::{crate_authors, crate_description, crate_name, crate_version};
use std::process;

fn main() {

    let matches = clap::App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            clap::Arg::with_name("WATCH_PATH")
                .long("path")
                .short("p")
                .default_value("")
                .help("directory of the watch path")
        )
        .arg(
            clap::Arg::with_name("LOG_LEVEL")
                .long("log_level")
                .short("l")
                .default_value("info")
                .help("log level: error|warn|info|debug|trace")
        )
        .get_matches();


    let log_level: log::Level = matches
        .value_of("LOG_LEVEL")
        .unwrap()
        .parse()
        .expect("unable to parse log level");

    debug!("log level is {}", log_level);

    simple_logger::init_with_level(log_level).unwrap();

    let watch_path = matches
        .value_of("WATCH_PATH")
        .unwrap();


   if watch_path == "" {
       error!("watch_path can be empty!");
       process::exit(1);
   }
    debug!("watch path is {}", watch_path);


    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(watch_path, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => debug!("{:?}", event),
            Err(e) => error!("watch error: {:?}", e),
        }
    }
}