#[macro_use]
extern crate simple_log;

use simple_log::LogConfigBuilder;
use std::collections::HashMap;
use std::{env, thread, time};
use sysinfo::System;
use util::configuration::get_configuration;

mod util;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let mut log_level: &str = "warn";
    if cfg!(debug_assertions) {
        log_level = "debug";
    }

    if args.len() > 1 {
        log_level = &args[1];
    }
    println!("Log Level: {}", log_level);

    const LOG_PATH: &str = "./log/file.log";
    const LOG_SIZE: u64 = 100;
    const ROLL_COUNT: u32 = 10;
    const EXECUTABLE_MONITOR: &str = "EXECUTABLE_MONITOR";
    const TIME_TO_WAIT: &str = "TIME_TO_WAIT";
    let config = LogConfigBuilder::builder()
        .path(LOG_PATH)
        .size(LOG_SIZE)
        .roll_count(ROLL_COUNT)
        .level(log_level)
        .output_file()
        .output_console()
        .build();

    simple_log::new(config)?;
    info!("Starting monitor process");
    if !sysinfo::IS_SUPPORTED_SYSTEM {
        warn!("This OS isn't supported (yet?).");
    } else {
        debug!("This OS is supported!");
        loop {
            info!("Checking process executing in the machine");
            let mut system_keys: HashMap<String, String> = HashMap::new();
            load_configuration(&mut system_keys, EXECUTABLE_MONITOR);
            load_configuration(&mut system_keys, TIME_TO_WAIT);

            trace!("{:?}", system_keys);
            let system_to_check = system_keys[EXECUTABLE_MONITOR].split(',');
            let time_to_sleep: u64 = match system_keys[TIME_TO_WAIT].trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    error!("Error to convert TIME_TO_WAIT to u64");
                    60000
                }
            };
            for monitor_system in system_to_check {
                let result_check = check_executable(monitor_system);
                match result_check {
                    Some(value) => {
                        if value {
                            info!("Service {} is ok", monitor_system);
                        } else {
                            warn!(
                                "Service {} not found need restart the service",
                                monitor_system
                            );
                        }
                    }
                    None => {
                        warn!(
                            "Service {} not found need restart the service",
                            monitor_system
                        );
                    }
                }
            }
            let time_to_wait = time::Duration::from_millis(time_to_sleep);
            debug!(
                "Waiting in seconds {} to check again",
                time_to_wait.as_secs()
            );
            thread::sleep(time_to_wait);
        }
    }
    Ok(())
}
fn check_executable(monitor_system: &str) -> Option<bool> {
    info!("Monitor exec: {}", monitor_system);
    let mut sys = System::new_all();

    sys.refresh_all();
    let mut result: bool = false;
    // Display processes ID, name na disk usage:
    for (pid, process) in sys.processes() {
        if process.name() == monitor_system {
            info!("Services is executing: {}", process.name());
            debug!(
                "[{pid}] {} {} {:?}",
                process.name(),
                process.status(),
                process.disk_usage()
            );
            result = true;
            break;
        }
    }
    if result {
        Some(result)
    } else {
        None
    }
}

fn load_configuration(map_key: &mut HashMap<String, String>, key: &str) {
    let result_keys = get_configuration(key);
    match result_keys {
        Some(data) => {
            map_key.insert(key.to_string(), data);
            trace!("{:?}",map_key);
        }
        None => {
            warn!("Key {} not found", key);
        }
    }
}
