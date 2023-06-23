use std::{fs, sync::mpsc::{channel, Receiver}, path::PathBuf, ffi::{OsString, OsStr}};
use chrono::{DateTime, Local, Duration};
use ini::Ini;
use timer::{Timer, Guard};

pub struct Sorter {
    timer: Timer,
    guard: Option<Guard>,
}

#[allow(unused)]
impl Sorter {
    pub fn new() -> Self {
        Self {
            timer: Timer::new(),
            guard: None,
        }
    }

    pub fn schedule_sorting(&mut self, conf: &Ini, date: DateTime<Local>, interval: Duration)
     -> Result<Receiver<bool>, String> {

        let conf = conf.clone();
        let (s, r) = channel();
        self.guard = Some(self.timer.schedule(date, Some(interval), move || {
            Sorter::sort(&conf);
            s.send(true).unwrap_or(());
        }));
        return Ok(r);
    }

    pub fn stop_scheduled_sorting(&mut self) {
        self.guard = None;
    }

    pub fn sort(conf: &Ini) -> () {
        //let path = PathBuf::from(conf.general_section().get("PathToDownloads").unwrap());
        let root_path = PathBuf::from("./test/");
        let sorted_path = PathBuf::from(root_path.as_path()).join("sorted");

        let filenames = get_filenames_in_dir(&root_path);

        for filename in filenames {
            let from = PathBuf::from(root_path.as_path()).join(filename.as_str()); 
            let mut to = PathBuf::from(sorted_path.as_path()).join(filename.as_str());
            to = make_filename_unique(to);
            eprintln!("Renamed to '{}'", to.file_name().unwrap_or(OsStr::new("unknown")).to_str().unwrap_or("unknown"));

            match fs::rename(from, to) {
                Ok(_) => eprintln!("Moved '{filename}'"),
                Err(e) => eprintln!("Unable to move '{filename}': {e}"),
            };
        }
    }
}


//------------------------------------------------ Utility Functions ------------------------------------------------//
fn get_filenames_in_dir(dir_path: &PathBuf) -> impl Iterator<Item = String>{
    fs::read_dir(dir_path.as_path())
    .unwrap()
    .filter_map(|x| {
        let p = x.unwrap().path();
        if p.is_file() { // filter files (not dirs)
            return Some(String::from(p.file_name().unwrap().to_str().unwrap()))
        } else {
            return None
        }
    })
}

/// Ensures that the given path does not lead to an existing file
/// 
/// Uses the following rules to determine the new path (in that order):
/// 1. Appends " - Copy" at the end of file.
/// 2. Appends " - Copy (2)" at the end of file.
/// 3. Appends " - Copy (3)" at the end of file.
/// ...
/// 
/// 
fn make_filename_unique(path: PathBuf) -> PathBuf{
    if !path.is_file() {
        return path;
    }

    let name = String::from(
        path.with_extension("")
            .file_name().unwrap()
            .to_str().unwrap()
    );
    let ext = match path.extension() {
        Some(e) => String::from(e.to_str().unwrap()),
        None => String::new(),
    };
    let cp = " - Copy";

    let mut i = 1;
    loop {
        let unique_filename: String;
        if i == 1 {
            unique_filename = format!("{name}{cp}.{ext}");
        } else {
            unique_filename = format!("{name}{cp} ({i}).{ext}");
        }

        let new_path = match path.parent() {
            Some(p) => PathBuf::from(p).join(unique_filename),
            None => PathBuf::new().join(unique_filename),
        };

        if !new_path.is_file() {
            return new_path;
        }

        i += 1;
    };
}