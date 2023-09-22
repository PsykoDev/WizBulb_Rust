use std::process::Command;

pub fn ip_mac(mac: Option<&String>) -> String {

        let formatted_mac: String = mac.expect("")
            .chars()
            .enumerate()
            .fold(String::new(), |mut acc, (i, c)| {
                if i > 0 && i % 2 == 0 {
                    acc.push(':');
                }
                acc.push(c);
                acc
            });

        let output = Command::new("arp")
            .arg("-a")
            .output()
            .expect("Error arp command");

        let output_str = String::from_utf8_lossy(&output.stdout);

    return if let Some(line) = output_str.lines().find(|line| line.contains(formatted_mac.as_str())) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let ip_address = parts[1];
            ip_address.replace("(", "").replace(")", "")
        } else { "".to_string() }
    } else { "".to_string() }
}
