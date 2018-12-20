extern crate bitcoin;
extern crate clap;
extern crate core;
extern crate rgb;
extern crate hyper;

use clap::{App, Arg, SubCommand};
use std::env::home_dir;
use std::path::Path;
use server::start_server;

pub mod database;
pub mod server;

fn main() {
    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    const AUTHORS: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");

    let matches = App::new("RGB - Bifrost Server")
        .version(VERSION.unwrap_or("<unknown>"))
        .author(AUTHORS.unwrap_or("<unknown>"))
        .about("Store and help relaying RGB transactions")
        .arg(Arg::with_name("datadir")
            .short("d")
            .long("datadir")
            .value_name("DIRECTORY")
            .help("Sets a data directory")
            .takes_value(true))
        .arg(Arg::with_name("port")
            .long("port")
            .short("p")
            .value_name("PORT")
            .required(true)
            .default_value("3000")
            .help("Set a new port"))
        .get_matches();

    let default_rgb_dir = home_dir().unwrap().join(".rgb-server");
    let datadir = Path::new(matches.value_of("datadir").unwrap_or(default_rgb_dir.to_str().unwrap()));
    println!("Using datadir: {:?}", datadir);

    let mut database = database::Database::new(datadir);

    start_server(String::from(matches.value_of("port").unwrap()), database);

    return;
}