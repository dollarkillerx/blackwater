use blackwater::*;
use structopt::StructOpt;

#[async_std::main]
async fn main() -> Result<()> {
    let opt:Params = Params::from_args();
    println!("{}",LOGO);
    if opt.ip == None {
        println!("Please -v");
        return Ok(())
    }


    Ok(())
}
