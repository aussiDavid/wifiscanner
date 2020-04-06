use crate::{Error, Wifi};
use std::env;
use std::process::Command;

mod split_network_list;
mod network_to_wifi;
mod parse_iw_dev_scan;
mod parse_iw_dev;

/// Returns a list of WiFi hotspots in your area - (Linux) uses `iw`
pub(crate) fn scan() -> Result<Vec<Wifi>, Error> {
    const PATH_ENV: &'static str = "PATH";
    let path_system = "/usr/sbin:/sbin";
    let path = env::var_os(PATH_ENV).map_or(path_system.to_string(), |v| {
        format!("{}:{}", v.to_string_lossy().into_owned(), path_system)
    });

    let output = Command::new("iw")
        .env(PATH_ENV, path.clone())
        .arg("dev")
        .output()
        .map_err(|_| Error::CommandNotFound)?;
    let data = String::from_utf8_lossy(&output.stdout);
    let interface = parse_iw_dev::run(&data)?;

    let output = Command::new("iw")
        .env(PATH_ENV, path)
        .arg("dev")
        .arg(interface)
        .arg("scan")
        .output()
        .map_err(|_| Error::CommandNotFound)?;
    if !output.status.success() {
        return Err(Error::CommandFailed(
            output.status,
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }
    let data = String::from_utf8_lossy(&output.stdout);
    parse_iw_dev_scan::run(&data)
}
