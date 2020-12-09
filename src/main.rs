use blackwater::*;
use structopt::StructOpt;

#[async_std::main]
async fn main() -> Result<()> {
    let opt:Params = Params::from_args();
    println!("{}",LOGO);
    if opt.ip == None {
        println!("Please -h");
        return Ok(())
    }

    println!("{:?}",opt.get_ports());
    Ok(())
}
