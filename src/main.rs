extern crate clap;

use clap::{App, Arg};
mod pinger;

struct IsUpCli {
    url: String,
}

impl IsUpCli {
    fn new() -> Self {
        let app = App::new("is_up")
            .version("1.0")
            .about("Pings to a server's url")
            .author("Alae Es-saki");
        let url_option = Arg::with_name("url")
            .short("u")
            .long("url")
            .takes_value(true)
            .help("the website url to ping")
            .required(true);
        let app = app.arg(url_option);

        let matches = app.get_matches();

        let url = matches
            .value_of("url")
            .expect("the url can't be None, it's required");

        IsUpCli {
            url: url.to_string(),
        }
    }
}

fn main() {
    let is_up_cli = IsUpCli::new();
    let url = is_up_cli.url;
    println!("pinging to {} ...", url);
    pinger::do_pings(&url);
}
