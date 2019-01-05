extern crate regex;
use regex::Regex;
use std::process::Command;

fn main() {
    let packet_size = 1500.0;
    let pings = 50;
    let output = Command::new("ping")
        .arg("-i")
        .arg("0.1")
        .arg("-c")
        .arg(pings.to_string())
        .arg("-s")
        .arg((packet_size - 8.0).to_string())
        .arg("www.rogers.com")
        .output()
        .expect("ping failed");

    assert!(output.status.success());

    let speed = calc_speed(String::from_utf8_lossy(&output.stdout).to_string(),
                           packet_size);

    println!("server: www.rogers.com");
    println!("packet_size: {} data bytes per ping", packet_size);
    println!("pings: {}", pings);
    println!("speed: {} Mbit/s", speed);
}

fn calc_speed(string: String, packet_size: f32) -> f32 {
    // round-trip min/avg/max/stddev = 10.059/15.414/27.162/5.146 ms
    let re = Regex::new(r"round-trip.*\d+\.\d+/(\d+\.\d+)/\d+\.\d+/.*").unwrap();

    let avg_time: f32 = match re.captures(&string) {
        Some(capture) => capture.get(1).unwrap().as_str().parse().unwrap(),
        None => 0.0
    };

    let mbyte_s = packet_size / avg_time;
    let mbit_s = mbyte_s * 8.0;

    mbit_s
}

