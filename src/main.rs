use std::borrow::Borrow;
use std::fmt::{Display, Debug};
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

#[derive(Debug)]
pub struct ChartInfo {
    pub name: String,
    pub chart_version: String,
    pub app_version: String,
    pub description: String
}

impl Display for ChartInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}, chart-version: {}, app-version: {}, description: {}", self.name, self.chart_version, self.app_version, self.description)
    }
}

impl ChartInfo {
    pub fn extract(str: &str) -> Vec<ChartInfo>{

        println!("{:?}", str);
        println!("Parsing string");

        let lines = str.split("\n").skip(1);
        let mut data: Vec<ChartInfo> = Vec::new();

        for line in lines {
            println!("CURRENT LINT");
            println!("{}", line);
            let elements = line.split("\t");

            for element in elements {
                println!("{:?}", element);
                let chart_info = ChartInfo{
                    name: String::from(""),
                    chart_version: String::from(""),
                    app_version: String::from(""),
                    description: String::from("")
                };
        
                data.push(chart_info);
            }
        }

     

        return data;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_add() {

        let output =    r#"NAME                                    CHART VERSION   APP VERSION     DESCRIPTION                                       
                            bitnami/nginx                           9.7.6            1.21.6               NGINX Open Source is a web server that can be a...
                            bitnami/nginx-ingress-controller            9.1.5              1.1.1               NGINX Ingress Controller is an Ingress controll...
                            bitnami/nginx-intel                      0.1.2             0.4.7               NGINX Open Source for Intel is a lightweight se...
                            bitnami/kong                                5.0.2              2.7.0               Kong is a scalable, open source API layer (aka ..."#;
    
        let chart_info = ChartInfo::extract(output);
        assert!(chart_info.len() > 0);
        assert!(chart_info.len() == 4);
    }

  




}
