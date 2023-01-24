use std::sync::Arc;
use async_trait::async_trait;
use crate::agent::driver::{DriverTrait, ReceiveError, SendingError};
use crate::agent::message::Message;

#[derive(Debug, Clone)]
pub struct FlumeDriver {
    sender: Arc<flume::Sender<Message>>,
    receiver: Arc<flume::Receiver<Message>>,
}

impl FlumeDriver {
    pub fn new() -> Self {
        let (tx, rx) = flume::unbounded();
        FlumeDriver {
            sender: Arc::new(tx),
            receiver: Arc::new(rx),
        }
    }
}

#[async_trait]
impl DriverTrait for FlumeDriver {
    async fn send(&self, message: Message) -> Result<(), SendingError> {
        match self.sender.send_async(message).await {
            Ok(_) => Ok(()),
            Err(e) => Err(SendingError::Custom(e.to_string())),
        }
    }

    async fn recv(&self) -> Result<Message, ReceiveError> {
        match self.receiver.recv_async().await {
            Ok(message) => Ok(message),
            Err(e) => match e {
                flume::RecvError::Disconnected => Err(ReceiveError::Disconnected)
            },
        }
    }
}