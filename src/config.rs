use ini::{Ini, ParseOption};
use directories::UserDirs;

pub fn get() -> Ini {
    let mut w_opt = ParseOption::default();
    w_opt.enabled_escape = false; // prevent escaping on backslashes
    match Ini::load_from_file_opt("conf.ini", w_opt) {
        Ok(r) => r, // return config from file
        Err(e) => {
            // config doesn't exist
            if e.to_string() == "The system cannot find the file specified. (os error 2)" {
                let conf = get_default();
                conf.write_to_file("conf.ini").unwrap();
                conf // return default config
            }

            // unknown error, panic
            else {
                panic!("{}", e);
            }
        },
    }
}

fn get_default() -> Ini{
    let mut conf = Ini::new();

    let ud = UserDirs::new().unwrap();
    let downloads_path = ud.download_dir().unwrap();
    conf.with_general_section()
        .set("PathToDownloads", downloads_path.as_os_str().to_str().unwrap());
    
    return conf;
}