use std::str::FromStr;

use crate::message::Message;

#[derive(Debug)]
pub struct Content(Vec<Message>);

impl FromStr for Content {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        tracing::debug!("Parsing content");
        if s.trim() == "" {
            return Ok(Self(Vec::new()));
        }
        s.split("\n\n")
            .map(Message::from_str)
            .collect::<Result<Vec<_>, _>>()
            .map(Self)
            .inspect(|_| tracing::debug!("Content parsed"))
    }
}

impl Content {
    pub fn iter(&self) -> impl Iterator<Item = &Message> {
        self.0.iter()
    }

    pub fn check(
        &self,
        errors: &mut Vec<(i32, Vec<(&str, &str)>)>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        tracing::debug!("Checking content");
        let failed = self.iter().map(|m| m.check(errors)).fold(
            (0, 0, 0),
            |(total, failed_messages, failed_lints), r| match r {
                Ok(_) => (total + 1, failed_messages, failed_lints),
                Err(n) => (total + 1, failed_messages + 1, failed_lints + n),
            },
        );
        tracing::debug!("Content checked");

        match failed {
            (total, 0, 0) => {
                tracing::info!(
                    "{} messages checked, 0 failed messages, 0 failed checks",
                    total
                );
                tracing::info!("Good job!");
                Ok(total)
            }
            (total, failed_messages, failed_lints) => {
                tracing::error!(
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
