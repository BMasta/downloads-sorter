use sorting::Sorter;
use config::Config;
use std::error::Error;

mod config;
mod sorting;

fn sort_loop(conf: Config) -> Result<(), Box<dyn Error>> {
    let time_str = conf.start_datetime.to_rfc2822();
    eprintln!("Sorting scheduled at {}", time_str);


    let mut sorter = Sorter::new();
    let recv = sorter.schedule_sorting(conf).unwrap();
    loop {
        match recv.recv() {
            // faulure to receive result means timer has stopped so we exit the loop
            Err(_) => return Ok(()), 
            Ok(r) => {
                match r {
                    true => eprintln!("Sorting finished successfully!"),
                    false => eprintln!("Sorting finished with errors!"),
                }
            },
        };
    }
}

fn main() {
    match config::get() {
        Ok(conf) => match sort_loop(conf) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Sorting: {}", e)
            },
        },
        Err(e) => {
            eprintln!("Config: {}", e)
        },
    };
    
}
