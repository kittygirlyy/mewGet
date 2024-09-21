use std::fs::File;
use std::io::{Write, Read};
use std::error::Error as StdError;
use reqwest::blocking::Client;

use crate::progress::create_progress_bar;
use crate::utils::{extract_filename_from_url, extract_filename_from_headers};

pub fn download_file(url: &str) -> Result<(), Box<dyn StdError>> {
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