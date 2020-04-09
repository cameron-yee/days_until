extern crate chrono;

use std::env;
use chrono::{NaiveDateTime, Duration};
use chrono::prelude::Utc;

fn run(date_string: &str) {
    let final_date_string: String = format!("{} 00:00:01", date_string);
    let parsed_date_string: NaiveDateTime = NaiveDateTime::parse_from_str(&final_date_string, "%m/%d/%y %H:%M:%S").unwrap();
    let now: NaiveDateTime = Utc::now().naive_utc();
    let duration: Duration = parsed_date_string.signed_duration_since(now) + Duration::days(1); // Cuts one day off
    
    println!("{} days until {}", duration.num_days(), date_string);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 2 {
       let date: &str = &args[1];
       run(date); 
    } else {
        println!("Please only supply a date in mm/dd/yy format.");
    }
}
