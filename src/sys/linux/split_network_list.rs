pub fn run(iw_output: &str) -> Vec<String> {
  return iw_output.replace("BSS", "BSS~~~~~~~~~~")
    .split("BSS")
    .into_iter()
    .filter(|line| !line.is_empty())
    .map(|line| line.replace("~~~~~~~~~~", "BSS"))
    .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_given_2_devices_it_finds_all_devices() {
        let first_device = "BSS 22:33:44:55:66:77(on wlp2s0) -- associated\n\
                            \tchannel: 10\n";
        let second_device = "BSS 11:22:33:44:55:66(on wlp2s1) -- associated\n\
                             \tchannel: 11\n";
				let fixture = format!("{}{}", first_device, second_device);

        let expected = vec![first_device, second_device];
				let actual = run(&fixture);

        assert_eq!(expected, actual);
    }

    #[test]
    fn when_given_3_devices_it_finds_all_devices() {
        let first_device = "BSS 22:33:44:55:66:77(on wlp2s0) -- associated\n\
                            \tchannel: 10\n";
        let second_device = "BSS 11:22:33:44:55:66(on wlp2s1) -- associated\n\
                             \tchannel: 11\n";
        let third_device = "BSS 00:22:44:66:11:11(on wlp2s2) -- associated\n\
                             \tchannel: 2\n";
				let fixture = format!("{}{}{}", first_device, second_device, third_device);

        let expected = vec![first_device, second_device, third_device];
				let actual = run(&fixture);

        assert_eq!(expected, actual);
    }

    #[test]
    fn when_given_3_devices_with_additional_references_to_BSS_it_finds_all_devices() {
        let first_device = "BSS 22:33:44:55:66:77(on wlp2s0) -- associated\n\
                            \tchannel: 10\n";
        let second_device = "BSS 11:22:33:44:55:66(on wlp2s1) -- associated\n\
                             \tchannel: 11\n
                             \t	BSS Load: high\n";
        let third_device = "BSS 00:22:44:66:11:11(on wlp2s2) -- associated\n\
                             \tchannel: 2\n";
				let fixture = format!("{}{}{}", first_device, second_device, third_device);

        let expected = vec![first_device, second_device, third_device];
				let actual = run(&fixture);

        assert_eq!(expected, actual);
    }
}
