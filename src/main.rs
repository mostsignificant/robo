mod config;

use config::Config;
use std::fs;

fn main() {
    let contents = fs::read_to_string("./examples/restapi.robo.yaml").unwrap();
    let config: Config = serde_yaml::from_str(&contents).unwrap();

    println!("{:?}", config);
}
