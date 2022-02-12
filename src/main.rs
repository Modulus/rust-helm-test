use std::borrow::Borrow;
use std::{process::Command, str};

use rust_helm_test::chartinfo:: ChartInfo;
fn main() {
    let result = Command::new("sh")
    .arg("-c")
    .arg("helm search repo nginx")
    .output();


    match result {
        Ok(output) => {
            println!("Status: {:?}", output.status.code());
            match str::from_utf8(output.stdout.borrow()) {
                Ok(v) => {
                    let data = ChartInfo::extract(v);
                    println!("{:?}", data);
                },
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
        }
        Err(err) => {
            println!("Failed {}!", err);
        }
    }
}
