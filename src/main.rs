use std::fs;
use notify_rust::Notification;

fn main() {
    let file_path = "/sys/devices/platform/asus-nb-wmi/fan_boost_mode";

    let modes: [&str; 3] = ["Silent", "Balanced", "Boost"];

    let mut previous_mode: usize = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .trim()
        .to_string()
        .parse::<usize>()
        .unwrap();

    println!("Current fan mode: {}", modes[previous_mode]);

    loop {
        let fan_mode: usize = fs::read_to_string(file_path)
            .expect("Should have been able to read the file")
            .trim()
            .to_string()
            .parse::<usize>()
            .unwrap();

        if fan_mode != previous_mode {
            println!("Current mode: {}", modes[fan_mode]);
            Notification::new()
                .summary(format!("Current fan mode: {}", modes[fan_mode]).as_str())
                .show()
                .unwrap();
        }
        previous_mode = fan_mode;
    }
}