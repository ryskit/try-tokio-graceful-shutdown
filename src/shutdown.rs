use tokio::sync::broadcast;
use tokio::time::{Duration, sleep};

#[derive(Debug)]
pub struct Shutdown {
    shutdown: bool,
    notify: broadcast::Receiver<()>,
}

impl Shutdown {
    pub fn new(notify: broadcast::Receiver<()>) -> Shutdown {
        Shutdown {
            shutdown: false,
            notify,
        }
    }

    pub fn is_shutdown(&self) -> bool {
        self.shutdown
    }

    pub async fn recv(&mut self) {
        if self.shutdown {
            return;
        }

        sleep(Duration::from_secs(10)).await;

        let _ = self.notify.recv().await;

        self.shutdown = true;
    }
}
