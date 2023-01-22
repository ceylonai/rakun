use std::sync::Arc;
use flume::{RecvError, SendError};
use crate::agent::message::Message;

#[derive(Debug, Clone)]
pub struct Driver {
    sender: Arc<flume::Sender<Message>>,
    receiver: Arc<flume::Receiver<Message>>,
}

impl Driver {
    pub fn new() -> Self {
        let (tx, rx) = flume::unbounded();
        Driver {
            sender: Arc::new(tx),
            receiver: Arc::new(rx),
        }
    }

    pub async fn send(&self, message: Message) -> Result<(), SendError<Message>> {
        self.sender.send_async(message).await
    }

    pub async fn recv(&self) -> Result<Message, RecvError> {
        self.receiver.recv_async().await
    }
}