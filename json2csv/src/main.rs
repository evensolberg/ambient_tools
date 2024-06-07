use std::fs::File;

fn main() {
    let file = File::open("weather_data/2024-05-01.json").expect("File not found");
    dbg!(&file);
}
