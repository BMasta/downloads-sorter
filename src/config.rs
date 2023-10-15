use chrono::DateTime;
use chrono::Duration;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Deserialize, Clone)]
struct ConfigJson {
    downloads_path: String,
    sort_at: String,
    every_n_days: i64
}

pub struct Config {
    pub downloads_path: PathBuf,
    pub start_datetime: DateTime<Local>,
    pub interval: Duration
}

pub fn get() -> Result<Config, Box< dyn Error>> {
    let conf_file = File::open("./config.json")?;
    let reader = BufReader::new(conf_file);
    let conf_raw: ConfigJson = serde_json::from_reader(reader)?;

    // downloads_path
    let downloads_path = PathBuf::from(conf_raw.downloads_path);
    if !downloads_path.is_dir() {
        return Err("Invalid path".into());
    }
    if conf_raw.every_n_days.is_negative() || conf_raw.every_n_days > 7 {
        return Err("Supported sorting interval is 1-7 days".into());
    }

    // start datetime
    let sort_time = NaiveTime::parse_from_str(&conf_raw.sort_at, "%I:%M%p").unwrap();
    let today = Local::now().date_naive();
    let naive_start = NaiveDateTime::new(today, sort_time);
    let start_datetime = naive_start.and_local_timezone(Local).unwrap();

    // interval
    let interval = Duration::days(conf_raw.every_n_days);

    let conf = Config {
        downloads_path: downloads_path,
        start_datetime: start_datetime,
        interval: interval
    };

    Ok(conf)
}