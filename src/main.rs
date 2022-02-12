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
            println!("{}", line);
            let elements : Vec<&str> = line.split("\t").collect();

            if elements.len() >= 4 {
                println!("Length is there!");

                let chart_info = ChartInfo{
                    name: String::from(elements[0].trim()),
                    chart_version: String::from(elements[1].trim()),
                    app_version: String::from(elements[2].trim()),
                    description: String::from(elements[3].trim())
                };
        
                data.push(chart_info);
            }  
            else {
                println!("Failed to insert element: {:?}", elements)
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
    fn test_extract() {

        let output =   "NAME                                    CHART VERSION   APP VERSION     DESCRIPTION       \n                             
                            bitnami/nginx\t                           9.7.6\t            1.21.6\t               NGINX Open Source is a web server that can be a...\n\
                            bitnami/nginx-ingress-controller\t        9.1.5\t            1.1.1\t               NGINX Ingress Controller is an Ingress controll...\n\
                            bitnami/nginx-intel\t                     0.1.2\t            0.4.7\t               NGINX Open Source for Intel is a lightweight se...\n\
                            bitnami/kong\t                            5.0.2\t            2.7.0\t               Kong is a scalable, open source API layer (aka ...\n";
    
        let chart_info = ChartInfo::extract(output);
        assert!(chart_info.len() > 0);
        assert!(chart_info.len() >= 4);

    }

  




}
