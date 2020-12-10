use super::*;
use async_std::channel;
use async_std::sync::Arc;
use async_std::task;

pub struct Core<'a> {
    param: &'a params::Params,
}

impl<'a> Core<'a> {
    pub async fn new(param: &'a params::Params) -> Core<'a> {
        Core {
            param
        }
    }

    pub async fn run(&mut self, ports: Vec<String>) -> Result<()> {
        let (sen_file, rec_file) = channel::unbounded();
        let output = Arc::new(Output::new(rec_file, self.param.outfile.clone()).await);

        // Concurrency Control
        let (sen_limit, rec_limit) = channel::bounded(self.param.concurrency as usize);
        let wg = Arc::new(WaitGroup::new().await);
        let ip = self.param.ip.as_ref().unwrap();

        for port in ports {
            sen_limit.send(1).await;
            wg.add().await;

            let wg = wg.clone();
            let rec_limit = rec_limit.clone();
            let ip = ip.clone();
            task::spawn(async move {
                println!("{}:{}",ip,port);
                wg.done().await;
                rec_limit.recv().await;
            });
        }

        wg.wait().await;

        Ok(())
    }
}