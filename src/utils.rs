use reqwest::Url;

pub fn extract_filename_from_url(url: &str) -> Option<String> {
    Url::parse(url)
        .ok()
        .and_then(|parsed_url| {
            parsed_url.path_segments().and_then(|segments| {
                segments.last().map(|filename| filename.to_string())
            })
        })
}

pub fn extract_filename_from_headers(response: &reqwest::blocking::Response) -> Option<String> {
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