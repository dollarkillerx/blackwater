use super::*;
use async_std::channel;
use async_std::sync::Arc;
use async_std::task;
use async_std::net::TcpStream;
use async_std::prelude::*;
use std::time::Duration;

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

        // run output
        task::spawn(async move {
            output.run().await;
        });

        // Concurrency Control
        let (sen_limit, rec_limit) = channel::bounded(self.param.concurrency as usize);
        let wg = Arc::new(WaitGroup::new().await);
        let ip = self.param.ip.as_ref().unwrap();
        let mut timeout = self.param.timeout;
        if timeout == 0 {
            timeout = 800
        }

        for port in ports {
            sen_limit.send(1).await.unwrap();
            wg.add().await;

            let wg = wg.clone();
            let rec_limit = rec_limit.clone();
            let sen_file = sen_file.clone();
            let ip = ip.clone();
            task::spawn(async move {
                match Self::blasting(format!("{}:{}", ip, port), timeout).await {
                    Ok(data) => {
                        sen_file.send(data).await.unwrap();
                    }
                    _ => {}
                }
                wg.done().await;
                rec_limit.recv().await.unwrap();
            });
        }

        wg.wait().await;

        Ok(())
    }

    async fn blasting(addr: String, timeout: u64) -> Result<String> {
        let conn: std::result::Result<async_std::net::TcpStream, std::io::Error> = TcpStream::connect(&addr).timeout(Duration::from_millis(timeout)).await?;
        match conn {
            Ok(_) => {
                Ok(addr)
            }
            _ => {
                Err("conn error".into())
            }
        }
    }
}