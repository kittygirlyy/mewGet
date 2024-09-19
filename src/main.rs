extern crate clap;

use clap::{Arg, App};
fn main() {
    let matches = App::new("mewGet")
        .version("1.0")
        .author("Kitty")
        .about("wget for cats")
        .arg(Arg::with_name("url")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("url to dl"))
        .get_matches();
    let mew = matches.value_of("url").unwrap();
    println!("{}", mew);
}
