extern crate dotenv;

use dotenv::dotenv;
use std:: env;

pub fn get_configuration(par_key: &str) -> Option<String> {
    info!("Searching value to key {}", par_key);
    dotenv().ok();    
    let mut return_data:String = String::new();
    for (key, value) in env::vars() {
        if key.eq(par_key) {
            debug!("Key found {} loading key to system", key);
            return_data = value;
        }
    }

    if !return_data.is_empty() {
        Some(return_data)
    } else {
        None
    }
}
