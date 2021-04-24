use super::*;
use std::sync::Arc;
use tokio::net::{TcpStream};
use tokio::sync::{mpsc, Mutex};
// use tokio::prelude::*;
use tokio::time;

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
        let (sen_file, rec_file) = mpsc::unbounded_channel();
        // let mut output = Arc::new(Output::new(rec_file, self.param.outfile.clone()));
        let mut output = Output::new(rec_file, self.param.outfile.clone());

        // run output
        tokio::spawn(async move {
            output.run().await;
        });

        // Concurrency Control
        let (sen_limit, rec_limit) = mpsc::channel(self.param.concurrency as usize);
        let rec_limit = Arc::new(Mutex::new(rec_limit));
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
            tokio::spawn(async move {
                match Self::blasting(format!("{}:{}", ip, port), timeout).await {
                    Ok(data) => {
                        sen_file.send(data).unwrap();
                        // sen_file.send(data).await.unwrap();
                    }
                    _ => {}
                }
                wg.done().await;

                rec_limit.lock().await.recv().await.unwrap();
            });
        }

        wg.wait().await;

        Ok(())
    }

    async fn blasting(addr: String, timeout: u64) -> Result<String> {
        // let conn: std::result::Result<async_std::net::TcpStream, std::io::Error> = TcpStream::connect(&addr).timeout(Duration::from_millis(timeout)).await?;
        let conn = time::timeout(
            time::Duration::from_millis(timeout),
            TcpStream::connect(&addr),
        ).await;

        match conn {
            Ok(r) => {
                if let Ok(_) = r {
                    return Ok(addr)
                };

                Err("conn error".into())
            }
            _ => {
                Err("conn error".into())
            }
        }
    }
}