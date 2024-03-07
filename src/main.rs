use std::{fs, thread, time};
use notify_rust::Notification;

fn main() {
    let modes: [&str; 3] = ["Boost", "Balanced", "Silent"];
    let mut previous_mode: usize = get_mode();

    loop {
        let fan_mode: usize = get_mode();

        if fan_mode != previous_mode {
            println!("Current mode: {}", modes[fan_mode]);
            Notification::new()
                .summary(format!("Current fan mode: {}", modes[fan_mode]).as_str())
                .show()
                .unwrap();
        }
        previous_mode = fan_mode;

        thread::sleep(time::Duration::from_secs(1));
    }
}

fn get_mode() -> usize {
    let file_path: &str = "/sys/devices/platform/asus-nb-wmi/fan_boost_mode";

    return fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .trim()
        .to_string()
        .parse::<usize>()
        .unwrap();
}
