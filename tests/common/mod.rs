//! Common test utilities
//!
//! This module provides test helpers like MockUnderlyingLayer for integration tests.

use async_trait::async_trait;
use futures::channel::mpsc;
use mqtt_client_wasm::{UnderlyingLayerCommand, UnderlyingLayerEvent, UnderlyingLayerInterface};
use std::collections::HashMap;
use tokio::task::JoinHandle;

/// Mock underlying layer for testing (pure message-passing)
pub struct MockUnderlyingLayer {
    pub event_sender: mpsc::UnboundedSender<UnderlyingLayerEvent>,
    event_receiver: Option<mpsc::UnboundedReceiver<UnderlyingLayerEvent>>,
    command_sender: mpsc::UnboundedSender<UnderlyingLayerCommand>,
    command_receiver: mpsc::UnboundedReceiver<UnderlyingLayerCommand>,
    connected: bool,
    sent_data: Vec<Vec<u8>>,
    /// Active timers: kind -> JoinHandle for the timer task
    active_timers: HashMap<String, JoinHandle<()>>,
}

#[allow(dead_code)]
impl MockUnderlyingLayer {
    pub fn new() -> Self {
        let (event_sender, event_receiver) = mpsc::unbounded();
        let (command_sender, command_receiver) = mpsc::unbounded();

        Self {
            event_sender,
            event_receiver: Some(event_receiver),
            command_sender,
            command_receiver,
            connected: false,
            sent_data: Vec::new(),
            active_timers: HashMap::new(),
        }
    }

    /// Simulate receiving data
    pub fn simulate_receive(&self, data: Vec<u8>) {
        let _ = self
            .event_sender
            .unbounded_send(UnderlyingLayerEvent::Message(data));
    }

    /// Simulate connection
    pub fn simulate_connect(&mut self) {
        self.connected = true;
        let _ = self
            .event_sender
            .unbounded_send(UnderlyingLayerEvent::Connected);
    }

    /// Simulate error
    pub fn simulate_error(&self, error: String) {
        let _ = self
            .event_sender
            .unbounded_send(UnderlyingLayerEvent::Error(error));
    }

    /// Simulate close
    pub fn simulate_close(&mut self) {
        self.connected = false;
        let _ = self
            .event_sender
            .unbounded_send(UnderlyingLayerEvent::Closed);
    }

    /// Get sent data for verification
    pub fn sent_data(&self) -> &[Vec<u8>] {
        &self.sent_data
    }
}

#[async_trait(?Send)]
impl UnderlyingLayerInterface for MockUnderlyingLayer {
    fn event_receiver(&mut self) -> mpsc::UnboundedReceiver<UnderlyingLayerEvent> {
        self.event_receiver.take().unwrap_or_else(|| {
            let (_, receiver) = mpsc::unbounded();
            receiver
        })
    }

    fn command_sender(&self) -> mpsc::UnboundedSender<UnderlyingLayerCommand> {
        self.command_sender.clone()
    }

    async fn run(&mut self) {
        use futures::stream::StreamExt;

        while let Some(command) = self.command_receiver.next().await {
            match command {
                UnderlyingLayerCommand::Connect(url, reply_arc) => {
                    println!("MockUnderlyingLayer connecting to: {}", url);
                    self.connected = true;

                    // Send reply to complete the connect() await
                    if let Ok(mut reply_opt) = reply_arc.lock() {
                        if let Some(reply) = reply_opt.take() {
                            let _ = reply.send(Ok(()));
                        }
                    }

                    let _ = self
                        .event_sender
                        .unbounded_send(UnderlyingLayerEvent::Connected);
                }
                UnderlyingLayerCommand::SendData(data) => {
                    if self.connected {
                        self.sent_data.push(data);
                    } else {
                        let _ = self
                            .event_sender
                            .unbounded_send(UnderlyingLayerEvent::Error(
                                "Not connected".to_string(),
                            ));
                    }
                }
                UnderlyingLayerCommand::Close => {
                    self.connected = false;

                    // Cancel all active timers on close
                    for (_, handle) in self.active_timers.drain() {
                        handle.abort();
                    }

                    let _ = self
                        .event_sender
                        .unbounded_send(UnderlyingLayerEvent::Closed);
                    // Don't break - allow reconnection by continuing to process commands
                }
                UnderlyingLayerCommand::TimerReset { kind, duration_ms } => {
                    println!(
                        "MockUnderlyingLayer: TimerReset {} for {}ms",
                        kind, duration_ms
                    );

                    // Cancel existing timer if any
                    if let Some(old_handle) = self.active_timers.remove(&kind) {
                        println!("MockUnderlyingLayer: Cancelling existing timer {}", kind);
                        old_handle.abort();
                    }

                    // Spawn a new timer task using tokio
                    let event_sender = self.event_sender.clone();
                    let timer_kind = kind.clone();
                    let handle = tokio::spawn(async move {
                        tokio::time::sleep(tokio::time::Duration::from_millis(duration_ms)).await;
                        println!("MockUnderlyingLayer: Timer expired: {}", timer_kind);
                        let _ = event_sender
                            .unbounded_send(UnderlyingLayerEvent::TimerExpired(timer_kind));
                    });

                    self.active_timers.insert(kind, handle);
                }
                UnderlyingLayerCommand::TimerCancel { kind } => {
                    println!("MockUnderlyingLayer: TimerCancel {}", kind);

                    if let Some(handle) = self.active_timers.remove(&kind) {
                        println!("MockUnderlyingLayer: Cancelling timer {}", kind);
                        handle.abort();
                    } else {
                        println!(
                            "MockUnderlyingLayer: Timer {} was not active, nothing to cancel",
                            kind
                        );
                    }
                }
            }
        }
    }
}
