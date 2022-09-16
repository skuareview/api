use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;
use log::{info, error, debug};
use uuid::Uuid;
use actix_web::web;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct Monitor {
    pub user_uuid: String,
    pub url: String,
    pub inactive_sec: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertableMonitor {
    pub monitor: Monitor,
    pub monitor_uuid: Uuid,
}

impl Monitor {
    // TODO: Maybe put this at the start of the api 
    fn check_systemd_units_path(path: &String) -> std::io::Result<()> { 
        match fs::create_dir_all(path) {
            Ok(..) => {
                info!("The {} path has been successfully created", path);
                return Ok(())
            },
            Err(e) => {
                error!("The {} path cannot be created", path);
                return Err(e)
            }
        }
    }

    fn write_systemd_file(path: &str, contents: &str) -> std::io::Result<()> {
        match File::create(&path) {
            Ok(mut file) => {
                match file.write_all(contents.as_bytes()) {
                    Ok(..) => {
                        debug!("The file has been written");
                        return Ok(())
                    },
                    Err(e) => {
                        error!("Unable to write to the file");
                        return Err(e)
                    }
                }
            }
            Err(e) => {
                error!("Unable to create the file");
                return Err(e)
            }
        };
    }

    pub fn contents_systemd_service(path: &String, insertable_monitor: &InsertableMonitor) -> std::io::Result<()> { 
        let path = format!("{}/mon_{}.service", path, insertable_monitor.monitor_uuid);
        let unit = format!("[Unit]\nDescription=Monitor owned by user {}\n\n", insertable_monitor.monitor.user_uuid);
        let service = format!("[Service]\nType=oneshot\nExecStart={}\n\n", insertable_monitor.monitor.url);
        let install = format!("[Install]\nWantedBy=multi-user.target");
        let contents = format!("{}{}{}", &unit, &service, &install);

        Monitor::write_systemd_file(&path, &contents)?;
        Ok(())
    }

    pub fn contents_systemd_timer(path: &String, insertable_monitor: &InsertableMonitor) -> std::io::Result<()> { 
        let path = format!("{}/mon_{}.timer", path, insertable_monitor.monitor_uuid);
        let unit = format!("[Unit]\nDescription=Monitor owned by user {}\n\n", insertable_monitor.monitor.user_uuid);
        let timer = format!("[Timer]\nOnStartupSec=1\nOnUnitInactiveSec={}\n\n", insertable_monitor.monitor.inactive_sec);
        let install = format!("[Install]\nWantedBy=multi-user.target");
        let contents = format!("{}{}{}", &unit, &timer, &install);

        Monitor::write_systemd_file(&path, &contents)?;
        Ok(())
    }

    pub fn systemd_timer_state(path: &String, insertable_monitor: &InsertableMonitor, state: &str) -> std::io::Result<()> { 
        let timer = format!("{}/mon_{}.service", path, insertable_monitor.monitor_uuid);

        Command::new("systemctl").args(["--user", state, "--now", &timer]).output().expect("Unable to start the timer");
        Ok(())
    }

    pub fn write_new_monitor(monitor: web::Json<Monitor>) -> std::io::Result<()> { 
        let path = String::from("/home/skuareview/.config/systemd/user");

        // Create insertable monitor with new uuid
        let insertable_monitor = InsertableMonitor {
            monitor: monitor.into_inner(),
            monitor_uuid: Uuid::new_v4()
        };

        // Check systemd units path
        Monitor::check_systemd_units_path(&path)?;

        // Create and write systemd service / timer contents
        Monitor::contents_systemd_service(&path, &insertable_monitor)?;
        Monitor::contents_systemd_timer(&path, &insertable_monitor)?;

        // Start timer
        Monitor::systemd_timer_state(&path, &insertable_monitor, "start")?;

        Ok(())
    }

}
