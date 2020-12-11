use blackwater::*;
use structopt::StructOpt;

#[async_std::main]
async fn main() -> Result<()> {
    let opt: Params = Params::from_args();
    println!("{}", LOGO);
    if opt.ip == None {
        println!("Please -h");
        return Ok(());
    }

    // println!("{:#?}",opt);

    let ports = opt.get_ports().await?;
    if ports.len() == 0 {
        println!("Parameter Error");
        return Ok(());
    }

    let mut core = Core::new(&opt).await;
    core.run(ports).await?;
    Ok(())
}
