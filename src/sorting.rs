use std::{fs::{self, create_dir_all}, sync::mpsc::{channel, Receiver}, path::PathBuf};
use std::collections::HashMap;
use timer::{Timer, Guard};

use crate::config::Config;

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

    pub fn schedule_sorting(&mut self, conf: Config)
     -> Result<Receiver<bool>, String> {
        let (s, r) = channel();
        self.guard = Some(self.timer.schedule(
            conf.start_datetime,
            Some(conf.interval),
            move || {
                let result = Sorter::sort(&conf);
                s.send(result);
            }));
        return Ok(r);
    }

    pub fn stop_scheduled_sorting(&mut self) {
        self.guard = None;
    }

    pub fn sort(conf: &Config) -> bool {
        let filenames = Sorter::get_filenames_in_dir(&conf.downloads_path);

        let mut is_success = true;
        for filename in filenames {
            let from = PathBuf::from(conf.downloads_path.as_path()).join(filename.as_str()); 
            let mut to = match Sorter::get_sorted_path(&from) {
                Some(sp) => sp,
                None => continue,
            };
            to = Sorter::make_filename_unique(to);
            

            match fs::rename(&from, &to) {
                Ok(_) =>  {
                    eprintln!("From: {}", &from.to_str().unwrap_or("unknown"));
                    eprintln!("To: {}\n", &to.to_str().unwrap_or("unknown"));
                },
                Err(e) => {
                    eprintln!("Unable to move '{filename}': {e}");
                    eprintln!("Destination: {}\n", &to.to_str().unwrap_or("unknown"));
                    is_success = false;
                },
            };
        }
        return is_success;
    }

    fn get_sorted_path(from: &PathBuf) -> Option<PathBuf> {
        let misc = "Misc";
        let docs = "Documents";
        let archives = "Archives";
        let execs = "Executables";
        let code = "Code";
        let images = "Images";
        let ext_to_dir_map = HashMap::from([
            ("pdf", docs),
            ("txt", docs),
            ("docx", docs),
            ("doc", docs),
            ("pptx", docs),
            ("ppt", docs),
            ("tex", docs),
            ("xlsx", docs),
            ("xls", docs),

            ("png", images),
            ("jpg", images),
            ("jpeg", images),
            ("svg", images),
            ("webp", images),

            ("rar", archives),
            ("zip", archives),
            ("7z", archives),

            ("html", code),
            ("css", code),
            ("csv", code),
            ("py", code),
            ("c", code),
            ("h", code),
            ("cpp", code),
            ("rs", code),
            ("csv", code),

            ("exe", execs),
            ("jar", execs),
            ("msi", execs),
            ("iso", execs),

            ("", misc),
        ]);

        let extension = match from.extension() {
            Some(ext) => ext.to_str().unwrap().to_lowercase(),
            None => String::from(""),
        };

        return match ext_to_dir_map.get(extension.as_str()) {
            Some(dir) =>  {
                let path = from.parent().unwrap();
                let basename = from.file_name().unwrap();
                let new_path = PathBuf::from(path).join(dir);
                create_dir_all(&new_path).unwrap();
                return Some(new_path.join(basename));
            },
            None => return None,
        };

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
}