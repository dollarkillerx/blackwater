mod params;

pub use params::Params;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub const LOGO: &str = r"
 _      _
| |    | |
| |_   | |   __
| | |  | |  |  |
| _ |  |_|  |  |
Black Water
Asynchronous Port Scanner written in rust

";


#[cfg(test)]
mod tests {
    #[test]
    fn str_p() {
        // let p1 = "90-100".to_string();
        let p1 = "90-100".to_string();
        let idx1 = p1.find("-").unwrap();
        println!("{}  {} ", &p1[..idx1], &p1[idx1 + 1..]);
        let p_i = *&p1[..idx1].parse::<i32>().unwrap();
        let p_e = *&p1[idx1 + 1..].parse::<i32>().unwrap();
        for i in p_i..=p_e {
            println!("{}",i);
        }
    }

    #[test]
    fn str_p1() {
        let p1 = "80,443,50,65,56 ,565".to_string();
        let b:Vec<&str> = p1.split(",").collect();
        for i in &b{
            // *i = "sadsad";
            println!("{}",i)
        }
        println!("{:?}",b);
    }
}
