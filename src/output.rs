use tokio::fs::File;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Output {
    rec: UnboundedReceiver<String>,
    outfile: Option<PathBuf>,
}

impl Output {
    pub fn new(rec: UnboundedReceiver<String>, outfile: Option<PathBuf>) -> Output {
        Output {
            rec,
            outfile,
        }
    }

    pub async fn run(&mut self) {
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
                Some(r) => {
                    match output {
                        None => {
                            println!("{}", r);
                        }
                        Some(_) => {
                            println!("{}", r);
                            let r = format!("{} \n", r);
                            output.as_mut().unwrap().write_all(r.as_bytes()).await.unwrap();
                            output.as_mut().unwrap().flush().await.unwrap();

                        }
                    }
                }
                None => {
                    break 'a;
                }
            }
        }
    }
}