use crate::lints::{Lint, Lints};
use std::str::FromStr;

#[derive(Debug)]
pub struct Message {
    id: i32,
    content: String,
}

impl FromStr for Message {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once('\n')
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "No newline found"))
            .and_then(|(id, content)| {
                id.parse()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
                    .map(|id| (id, content.to_string()))
            })
            .map(|(id, content)| Message { id, content })
    }
}

impl Message {
    pub fn check(&self) -> Result<(), usize> {
        log::trace!("Checking message {}", self.id);
        let failed = Lints::check_on(&self.content);

        if failed.is_empty() {
            log::debug!("Message {} valid", self.id);
            Ok(())
        } else {
            log::warn!("Message {} invalid", self.id);
            failed
                .iter()
                .for_each(|l| log::warn!("{}: {}", l.code(), l.message()));
            log::warn!("");
            Err(failed.len())
        }
    }
}
