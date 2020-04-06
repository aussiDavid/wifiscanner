use crate::{Error};

pub fn run(interface: &str) -> Result<String, Error> {
    interface
        .split("\tInterface ")
        .take(2)
        .last()
        .ok_or(Error::NoValue)?
        .split("\n")
        .nth(0)
        .ok_or(Error::NoValue)
        .map(|text| text.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;

    #[test]
    fn when_something_happens_it_extracts_the_interface_id() {
        let expected = "wlp2s0";

        // FIXME: should be a better way to create test fixtures
        let mut path = PathBuf::new();
        path.push("tests");
        path.push("fixtures");
        path.push("iw");
        path.push("iw_dev_01.txt");

        let file_path = path.as_os_str();

        let mut file = File::open(&file_path).unwrap();

        let mut filestr = String::new();
        let _ = file.read_to_string(&mut filestr).unwrap();

        let result = run(&filestr).unwrap();
        assert_eq!(expected, result);
    }
}