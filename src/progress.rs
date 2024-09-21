use indicatif::{ProgressBar, ProgressStyle};

pub fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
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