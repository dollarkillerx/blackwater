use async_std::channel::Receiver;
use async_std::fs::File;
use async_std::prelude::*;
use std::path::PathBuf;

pub struct Output {
    rec: Receiver<String>,
    outfile: Option<PathBuf>,
}

impl Output {
    pub async fn new(rec: Receiver<String>, outfile: Option<PathBuf>) -> Output {
        Output {
            rec,
            outfile,
        }
    }

    pub async fn run(&self) {
        let output: Option<File>;
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
                            println!("{}", r);
                            let r = format!("{} \n", r);
                            output.as_ref().unwrap().write_all(r.as_bytes()).await.unwrap();
                        }
                    }
                }
                Err(_) => {
                    break 'a;
                }
            }
        }
    }
}