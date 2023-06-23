use chrono::{self, Duration};
use ini::Ini;
use sorting::Sorter;

mod config;
mod sorting;

fn sort_loop(conf: &Ini) -> () {
    let date = chrono::offset::Local::now().checked_add_signed(Duration::seconds(1)).unwrap();
    let interval = Duration::seconds(10);

    let mut sorter = Sorter::new();
    let recv = sorter.schedule_sorting(&conf, date, interval).unwrap();

    //let mut i = 0;
    loop {
        match recv.recv() {
            // faulure to receive result means timer has stopped so we exit the loop
            Err(_) => return, 
            Ok(r) => {
                match r {
                    true => eprintln!("Sorting finished successfully!"),
                    false => println!("Sorting finished with errors!"),
                }
            },
        };
        // i += 1;
        // if i == 5 {
        //     sorter.stop_scheduled_sorting();
        //     println!("Stopped scheduling")
        // }
    }
}

fn main() {
    let conf = config::get();
    sort_loop(&conf)
}
