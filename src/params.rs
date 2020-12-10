use structopt::StructOpt;
use std::path::PathBuf;
use super::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "blackwater", about = "Asynchronous Port Scanner written in rust  https://github.com/dollarkillerx/blackwater")]
pub struct Params {
    // Scanned IP address
    #[structopt(short = "i", long = "ip")]
    pub ip: Option<String>,

    // Number of concurrent scans
    #[structopt(short = "c", long = "concurrency", default_value = "20000")]
    pub concurrency: u32,

    // Port Range <port,port,port> or <port-port>
    #[structopt(short = "p", long = "port", default_value = "21,22,23,25,69,79,80,88,110,113,119,220,443,1433,1521,2082,2083,2086,2087,2095,2096,2077,2078,3306,3389,5432,6379,8080,9000,9001,9200,9300,11211,27017")]
    pub port: String,

    // Result output file address
    #[structopt(short = "f", long = "outfile", parse(from_os_str))]
    pub outfile: Option<PathBuf>,

    // Scanning with UDP
    #[structopt(short = "u", long = "udp")]
    pub udp: bool,
}

impl Params {
    pub async fn get_ports(&self) -> Result<Vec<String>> {
        let idx1 = match self.port.find("-") {
            Some(idx) => idx,
            None=> 0,
        };

        let idx2 = match self.port.find(",") {
            Some(idx) => idx,
            None=> 0,
        };

        if idx1 == 0 && idx2 == 0 {
            return match self.port.parse::<i32>() {
                Ok(i) => {
                    Ok(vec![format!("{}", i)])
                }
                Err(_) => {
                    Err("Parameter Error".into())
                }
            };
        }
        let mut lists = Vec::new();

        // param1
        if idx1 != 0 {
            let start = *&self.port[..idx1].parse::<i32>().unwrap();
            let end = *&self.port[idx1 + 1..].parse::<i32>().unwrap();
            for i in start..=end {
                lists.push(format!("{}",i));
            }
            return Ok(lists)
        }

        // param2
        let sli:Vec<&str> = self.port.split(",").collect();
        for i in sli {
            lists.push(i.trim().to_string());
        }

        Ok(lists)
    }
}

