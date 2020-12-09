use async_std::channel::Receiver;
use async_std::fs::File;
use async_std::prelude::*;

struct Output {
    rec: Receiver<String>,
    outfile: Option<String>,
}

impl Output {
    pub async fn new(rec: Receiver<String>, outfile: Option<String>) -> Output {
        Output {
            rec,
            outfile,
        }
    }

    pub async fn run(&self) {
        let mut output: Option<File>;
        match &self.outfile {
            None => {
                output = None
            }
            Some(file) => {
                output = Some(File::create(file).await.unwrap());
            }
        };

        'a:
        loop {
            match self.rec.recv().await {
                Ok(r) => {
                    match &self.outfile {
                        None => {
                            println!("{}", r);
                        }
                        Some(..) => {
                            (&output).as_ref().unwrap().write(r.as_bytes()).await;
                        }
                    }
                }
                Err(e) => {
                    break 'a;
                }
            }
        }
    }
}