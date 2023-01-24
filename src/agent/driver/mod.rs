mod flume_driver;

use std::sync::Arc;
use async_trait::async_trait;
use crate::agent::driver::flume_driver::FlumeDriver;
use crate::agent::message::Message;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReceiveError {
    RecvError,
    Disconnected,
    Timeout,
    Custom(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SendingError {
    Disconnected,
    Timeout,
    Custom(String),
}


#[async_trait]
pub trait DriverTrait {
    async fn send(&self, message: Message) -> Result<(), SendingError>;
    async fn recv(&self) -> Result<Message, ReceiveError>;
}


#[derive(Debug, Clone)]
pub struct DriverManager {
    driver: Arc<FlumeDriver>,
}

impl Default for DriverManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DriverManager {
    pub fn new() -> Self {
        DriverManager {
            driver: Arc::new(FlumeDriver::new()),
        }
    }

    pub async fn send(&self, message: Message) -> Result<(), SendingError> {
        self.driver.send(message).await
    }

    pub async fn recv(&self) -> Result<Message, ReceiveError> {
        self.driver.recv().await
    }
}