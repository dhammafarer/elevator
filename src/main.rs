use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

pub fn run_simulation() -> () {
    let mut location: f64 = 0.0;
    let mut velocity: f64 = 0.0;
    let mut acceleration: f64 = 0.0;

    let mut up_input_voltage: f64 = 0.0;
    let mut down_input_voltage: f64 = 0.0;

    let mut floor_count: u64 = 0;
    let mut floor_height: f64 = 0.0;
    let mut floor_requests: Vec<u64> = Vec::new();

    //4. Parse input and store as building description and floor
    let buffer = match env::args().nth(1) {
        Some(ref fp) if *fp == "-".to_string() => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("read_line failed");
            buffer
        }
        None => {
            let fp = "test.txt";
            let mut buffer = String::new();
            File::open(fp)
                .expect("File::open failed")
                .read_to_string(&mut buffer)
                .expect("read_to_string failed");
            buffer
        }
        Some(fp) => {
            let mut buffer = String::new();
            File::open(fp)
                .expect("File::open failed")
                .read_to_string(&mut buffer)
                .expect("read_to_string failed");
            buffer
        }
    };
    for (li, l) in buffer.lines().enumerate() {
        if li == 0 {
            floor_count = l.parse::<u64>().unwrap();
        } else if li == 1 {
            floor_height = l.parse::<f64>().unwrap();
        } else {
            floor_requests.push(l.parse::<u64>().unwrap());
        }
    }
    //5. Loop while there are remaining floor requests
    while floor_requests.len() > 0 {}

    //6. Print summary
    println!("summary");
}

fn main() {
    run_simulation()
}
