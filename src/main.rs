use std::{fs, thread, time};
use notify_rust::Notification;

fn main() {
    let file_path: &str = "/sys/devices/platform/asus-nb-wmi/fan_boost_mode";

    let modes: [&str; 3] = ["Silent", "Balanced", "Boost"];

    let wait: time::Duration = time::Duration::from_secs(1);

    let mut previous_mode: usize = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .trim()
        .to_string()
        .parse::<usize>()
        .unwrap();

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

        thread::sleep(wait);
    }
}