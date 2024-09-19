use reqwest::Url;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use std::fs::File;
use std::io::{Write, Read};
use std::error::Error as StdError;
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

    if let Err(e) = download_file(mew) {
        eprintln!("err lors du dl : {}", e);
    }
}

fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    let bar = match quiet_mode {
        true => ProgressBar::hidden(),
        false => match length {
            Some(len) => ProgressBar::new(len),
            None => ProgressBar::new_spinner(),
        },
    };

    bar.set_message(msg.to_string());

    if let Some(_) = length {
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}) eta: {eta}")
                .expect("err de configuration de la barre")
                .progress_chars("=> "),
        );
    } else {
        bar.set_style(ProgressStyle::default_spinner());
    }

    bar
}

fn extract_filename_from_url(url: &str) -> Option<String> {
    Url::parse(url)
        .ok()
        .and_then(|parsed_url| {
            parsed_url.path_segments().and_then(|segments| {
                segments.last().map(|filename| filename.to_string())
            })
        })
}

fn extract_filename_from_headers(response: &reqwest::blocking::Response) -> Option<String> {
    response.headers().get(reqwest::header::CONTENT_DISPOSITION).and_then(|header_value| {
        header_value.to_str().ok().and_then(|header| {
            let parts: Vec<&str> = header.split(";").collect();
            for part in parts {
                if part.trim().starts_with("filename=") {
                    return Some(part.trim().replace("filename=", "").replace("\"", ""));
                }
            }
            None
        })
    })
}

fn download_file(url: &str) -> Result<(), Box<dyn StdError>> {
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;

    let response = client.get(url).send()?;

    let total_size = response
        .content_length()
        .ok_or("taille du fichier introuvable")?;

    let filename = extract_filename_from_headers(&response)
        .or_else(|| extract_filename_from_url(url))
        .unwrap_or("fichier_inconnu".to_string());

    let pb = create_progress_bar(false, "dl en cours", Some(total_size));

    let mut dest = File::create(&filename)?;

    let mut downloaded: u64 = 0;
    let mut buffer = vec![0; 8192];

    let mut stream = response;

    while let Ok(n) = stream.read(&mut buffer) {
        if n == 0 {
            break;
        }
        dest.write_all(&buffer[..n])?;
        downloaded += n as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("dl fini");

    println!("fichier dl sous : {}", filename);

    Ok(())
}
