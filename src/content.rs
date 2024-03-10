use std::{path::Path, str::FromStr};

use crate::message::Message;

#[derive(Debug)]
pub struct Content(Vec<Message>);

impl FromStr for Content {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        log::debug!("Parsing content");
        s.split("\n\n")
            .map(Message::from_str)
            .collect::<Result<Vec<_>, _>>()
            .map(Self)
            .inspect(|_| log::debug!("Content parsed"))
    }
}

impl Content {
    pub fn from_path(path: &Path) -> Result<Content, std::io::Error> {
        log::debug!("Reading file {}", path.display());
        std::fs::read_to_string(path)
            .and_then(|s| s.parse())
            .inspect(|_| log::debug!("File read"))
    }

    pub fn iter(&self) -> impl Iterator<Item = &Message> {
        self.0.iter()
    }

    pub fn check(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::debug!("Checking content");
        let failed = self.iter().map(|m| m.check()).fold(
            (0, 0, 0),
            |(total, failed_messages, failed_lints), r| match r {
                Ok(_) => (total + 1, failed_messages, failed_lints),
                Err(n) => (total + 1, failed_messages + 1, failed_lints + n),
            },
        );
        log::debug!("Content checked");

        match failed {
            (total, 0, 0) => {
                log::info!(
                    "{} messages checked, 0 failed messages, 0 failed checks",
                    total
                );
                log::info!("Good job!");
                Ok(())
            }
            (total, failed_messages, failed_lints) => {
                log::error!(
                    "{} messages checked, {} failed messages, {} failed checks",
                    total,
                    failed_messages,
                    failed_lints
                );
                Err("Invalid content found".into())
            }
        }
    }
}
