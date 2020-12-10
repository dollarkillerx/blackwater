use super::*;
use async_std::channel;
use async_std::sync::{Arc, Mutex};
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

        for i in ports {
            sen_limit.send(1).await;
            wg.add().await;



            let wg = wg.clone();
            let rec_limit = rec_limit.clone();
            task::spawn(async move {
                wg.done().await;
                rec_limit.recv().await;
            });
        }

        wg.wait();

        Ok(())
    }
}