use crate::{Error, Wifi};

pub fn run(network_list: &str) -> Result<Vec<Wifi>, Error> {
    let wifis: Vec<Wifi> = super::split_network_list::run(network_list).into_iter()
        .map(|device| super::network_to_wifi::run(&device).unwrap())
        .collect();

    Ok(wifis)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;

    #[test]
    fn when_parsing_devices_it_gets_the_first_device() {
        let mut expected = Wifi {
            mac: "11:22:33:44:55:66".to_string(),
            ssid: "hello".to_string(),
            channel: "10".to_string(),
            signal_level: "-67.00".to_string(),
            security: "".to_string(),
        };

        // FIXME: should be a better way to create test fixtures
        let mut path = PathBuf::new();
        path.push("tests");
        path.push("fixtures");
        path.push("iw");
        path.push("iw_dev_scan_01.txt");

        let file_path = path.as_os_str();

        let mut file = File::open(&file_path).unwrap();

        let mut filestr = String::new();
        file.read_to_string(&mut filestr).unwrap();

        let result = run(&filestr).unwrap();
        assert_eq!(expected, result[0]);
    }

    #[test]
    fn when_parsing_devices_it_gets_the_sixth_device() {
        let mut expected = Wifi {
            mac: "66:77:88:99:aa:bb".to_string(),
            ssid: "hello-world-foo-bar".to_string(),
            channel: "8".to_string(),
            signal_level: "-89.00".to_string(),
            security: "".to_string(),
        };

        // FIXME: should be a better way to create test fixtures
        let mut path = PathBuf::new();
        path.push("tests");
        path.push("fixtures");
        path.push("iw");
        path.push("iw_dev_scan_01.txt");

        let file_path = path.as_os_str();

        let mut file = File::open(&file_path).unwrap();

        let mut filestr = String::new();
        file.read_to_string(&mut filestr).unwrap();

        let result = run(&filestr).unwrap();
        assert_eq!(expected, result[5]);
    }
}
