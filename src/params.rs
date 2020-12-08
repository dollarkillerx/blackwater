use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "blackwater", about = "Asynchronous Port Scanner written in rust")]
pub struct Params {
    // Scanned IP address
    #[structopt(short = "i", long = "ip")]
    pub ip: Option<String>,

    // Number of concurrent scans
    #[structopt(short = "c", long = "concurrency", default_value = "100", parse(from_occurrences))]
    pub concurrency: u32,

    // Port Range <port,port,port> or <port-port>
    #[structopt(short = "p", long = "port", default_value = "21,22,23,25,69,79,80,88,110,113,119,220,443,1433,1521,2082,2083,2086,2087,2095,2096,2077,2078,3306,3389,5432,6379,8080,9000,9001,9200,9300,11211,27017")]
    pub port: String,

    // Result output file address
    #[structopt(short = "f", long = "outfile", default_value = "outfile.txt", parse(from_os_str))]
    pub outfile: PathBuf,

    // Scanning with UDP
    #[structopt(short = "u", long = "udp")]
    pub udp: bool,
}