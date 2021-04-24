use tokio::sync::{Mutex};
use tokio::time;
use std::cell::Cell;

pub struct WaitGroup {
    // Remaining Tasks
    task: Mutex<Cell<u64>>
}

impl WaitGroup {
    pub async fn new() -> WaitGroup {
        WaitGroup {
            task: Mutex::new(Cell::new(0)),
        }
    }

    pub async fn add(&self) {
        let task = self.task.lock().await;
        task.set(task.get() + 1);
    }

    pub async fn done(&self) {
        let task = self.task.lock().await;
        task.set(task.get() - 1);
    }

    pub async fn wait(&self) {
        loop {
            let tk = self.task.lock().await;
            if tk.get() == 0 {
                break;
            }

            time::sleep(time::Duration::from_millis(100)).await;
        }
    }
}


