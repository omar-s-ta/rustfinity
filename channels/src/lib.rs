use core::fmt;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "LOW"),
            Priority::Medium => write!(f, "MED"),
            Priority::High => write!(f, "HIGH"),
            Priority::Critical => write!(f, "CRIT"),
        }
    }
}

pub struct Message {
    pub content: String,
    pub sender_id: u32,
    pub priority: Priority,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}|{}] {}", self.priority, self.sender_id, self.content)
    }
}

pub fn create_message_channel() -> (Sender<Message>, Receiver<Message>) {
    mpsc::channel()
}

pub fn create_producer_thread(messages: Vec<Message>, tx: Sender<Message>) -> JoinHandle<()> {
    thread::spawn(move || {
        for mut message in messages {
            if message.content.contains("ERROR") {
                message.priority = Priority::Critical;
            } else if message.content.contains("WARNING") {
                message.priority = Priority::High;
            } else if message.content.contains("DEBUG") {
                message.priority = Priority::Medium;
            } else {
                message.priority = Priority::Low;
            }
            tx.send(message).unwrap()
        }
    })
}

pub fn create_consumer_thread(rx: Receiver<Message>) -> JoinHandle<Vec<String>> {
    thread::spawn(move || {
        let mut messages = Vec::new();
        while let Ok(message) = rx.recv() {
            messages.push(format!("{message}"));
        }
        messages
    })
}

// Example Usage
pub fn main() {
    let (tx, rx) = create_message_channel();

    let mut producer_handles = vec![];
    for id in 0..3 {
        let tx_clone = tx.clone();
        let messages = vec![
            Message {
                content: format!("Normal message from producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
            Message {
                content: format!("WARNING: System running hot on producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
            Message {
                content: format!("ERROR: Connection lost on producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
        ];
        let handle = create_producer_thread(messages, tx_clone);
        producer_handles.push(handle);
    }

    drop(tx);
    let consumer_handle = create_consumer_thread(rx);

    for handle in producer_handles {
        handle.join().unwrap();
    }

    let results = consumer_handle.join().unwrap();
    println!("Processed messages:");
    for msg in results {
        println!("{}", msg);
    }
}
