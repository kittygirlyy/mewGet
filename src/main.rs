mod download;
mod progress;
mod utils;

use clap::App;

fn main() {
    let matches = App::new("mewGet")
        .version("1.0")
        .author("Kitty")
        .about("wget for cats")
        .arg(clap::Arg::with_name("url")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("url to dl"))
        .get_matches();

    let mew = matches.value_of("url").unwrap();

    if let Err(e) = download::download_file(mew) {
        eprintln!("err lors du dl : {}", e);
    }
}
