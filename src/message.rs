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
    pub fn check(&self, errors: &mut Vec<(i32, Vec<(&str, &str)>)>) -> Result<(), usize> {
        tracing::trace!("Checking message {}", self.id);
        let failed = Lints::check_on(&self.content);

        if failed.is_empty() {
            tracing::debug!("Message {} valid", self.id);
            Ok(())
        } else {
            tracing::debug!("Message {} invalid", self.id);

            let err = failed.iter().map(|l| (l.code(), l.message())).collect();
            errors.push((self.id, err));
            Err(failed.len())
        }
    }
}
