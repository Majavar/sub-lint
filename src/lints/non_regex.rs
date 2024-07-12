use super::Lint;

#[derive(Debug, Default)]
pub struct CharMaxPerLine;

impl Lint for CharMaxPerLine {
    fn code(&self) -> &'static str {
        "NX1-001"
    }

    fn message(&self) -> &'static str {
        "16 characters max per line"
    }

    fn lint(&self, s: &str) -> bool {
        !s.lines()
            .any(|l| l.chars().filter(|c| !c.is_whitespace()).count() > 16)
    }
}

#[derive(Debug, Default)]
pub struct LineCount;

impl Lint for LineCount {
    fn code(&self) -> &'static str {
        "NX10-001"
    }

    fn message(&self) -> &'static str {
        "Maximum two lines"
    }

    fn lint(&self, s: &str) -> bool {
        s.lines().count() <= 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_count() {
        let l = LineCount::default();
        assert!(l.lint("This is line 1"));
        assert!(l.lint("This is line 1\nThis is line 2"));
        assert!(!l.lint("This is line 1\nThis is line 2\nThis is line 3"));
    }

    #[test]
    fn test_char_max_per_line() {
        let l = CharMaxPerLine::default();
        assert!(l.lint("This line has 16\nThis has less"));
        assert!(!l.lint("This line has more than 16 characters"));
    }
}
