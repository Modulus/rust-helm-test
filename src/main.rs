use std::borrow::Borrow;
use std::process::Output;
use std::{process::Command, any::Any, str};
fn main() {
    let result = Command::new("sh")
    .arg("-c")
    .arg("helm search repo nginx")
    .output();


    match result {
        Ok(output) => {
            println!("Status: {:?}", output.status.code());
            let s = match str::from_utf8(output.stdout.borrow()) {
                Ok(v) => println!("{:?}", v),
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
        }
        Err(err) => {
            println!("Failed!");
        }
    }
}
