use crate::{Error, Wifi};

pub fn run(network: &str) -> Result<Wifi, Error> {
    let mut wifi = Wifi::default();

    for line in network.split("\n") {
        if let Ok(mac) = extract_value(line, "BSS ", Some("(")) {
            wifi.mac = mac;
        } else if let Ok(signal) = extract_value(line, "\tsignal: ", Some(" dBm")) {
            wifi.signal_level = signal;
        } else if let Ok(channel) = extract_value(line, "\tDS Parameter set: channel ", None) {
            wifi.channel = channel;
        } else if let Ok(ssid) = extract_value(line, "\tSSID: ", None) {
            wifi.ssid = ssid;
        } else if let Ok(security) = extract_value(line, "\t", Some(":")) {
            if security == "WPA" {
                wifi.security = security;
            }
        }
    }

    Ok(wifi);
}

fn extract_value(
    line: &str,
    pattern_start: &str,
    pattern_end: Option<&str>,
) -> Result<String, Error> {
    let start = pattern_start.len();
    if start < line.len() && &line[0..start] == pattern_start {
        let end = match pattern_end {
            Some(end) => line.find(end).ok_or(Error::NoValue)?,
            None => line.len(),
        };
        Ok(line[start..end].to_string())
    } else {
        Err(Error::NoValue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn when_given_one_device_it_parses_the_mac_address() {
        let expected = "11:22:33:44:55:66";
        let actual = run(&fixture()).unwrap().mac;

        assert_eq!(expected, actual);
    }

    #[test]
    fn when_given_one_device_it_parses_the_ssid() {
        let expected = "hello";
        let actual = run(&fixture()).unwrap().ssid;

        assert_eq!(expected, actual);
    }

    #[test]
    fn when_given_one_device_it_parses_the_channel() {
        let expected = "10";
        let actual = run(&fixture()).unwrap().channel;

        assert_eq!(expected, actual);
    }

    #[test]
    fn when_given_one_device_it_parses_the_signal_level() {
        let expected = "-67.00";
        let actual = run(&fixture()).unwrap().signal_level;

        assert_eq!(expected, actual);
    }

    #[test]
    fn when_given_one_device_it_parses_the_security() {
        let expected = "WPA";
        let actual = run(&fixture()).unwrap().security;

        assert_eq!(expected, actual);
    }

    fn fixture() -> std::string::String {
        return fs::read_to_string("tests/fixtures/iw/iw_dev_scan_02.txt")
            .expect("Something went wring reading the fixture");
    }
}
