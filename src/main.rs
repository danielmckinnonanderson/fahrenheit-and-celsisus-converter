use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    parse_args(&args);
}

fn parse_args(args: &Vec<String>) {
    match args[1].trim() {
        "f" => println!("{}", fahrenheit_to_celsius(args[2].parse().unwrap())),
        "c" => println!("{}", celsius_to_fahrenheit(args[2].parse().unwrap())),
        _ => panic!("Expected 'f' or 'c' as first argument!"),
    };
}

fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) / 1.8
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    (temp * 1.8) + 32.0
}
