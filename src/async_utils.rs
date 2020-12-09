use async_std::sync::{Mutex};
use async_std::task;

struct WaitGroup {
    // Remaining Tasks
    task: Mutex<u64>
}

impl WaitGroup {
    pub async fn new() -> WaitGroup {
        WaitGroup{
            task: Mutex::new(0),
        }
    }

    pub async fn add(&mut self) {
        let mut task = self.task.lock().await;
        *task = *task + 1;
    }

    pub async fn done(&mut self) {
        let mut task = self.task.lock().await;
        *task = *task - 1;
    }

    pub async fn wait(&mut self) {
        loop {
            let tk = self.task.lock().await;
            if *tk == 0 {
                break
            }

            task::sleep(std::time::Duration::from_millis(200)).await;
        }
    }
}


