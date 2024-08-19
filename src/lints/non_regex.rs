use super::{Lint, PUNCTUATION};

fn len(s: &str) -> f32 {
    s.chars()
        .map(|c| if PUNCTUATION.contains(c) { 0.5 } else { 1.0 })
        .sum()
}

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
        !s.lines().any(|l| len(l) > 16.0)
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

#[derive(Debug, Default)]
pub struct FitOnOneLine;

impl Lint for FitOnOneLine {
    fn code(&self) -> &'static str {
        "LXX-005"
    }

    fn message(&self) -> &'static str {
        "Can fit on one line"
    }

    fn lint(&self, s: &str) -> bool {
        s.lines().count() == 1
            || s.lines().all(|l| l.as_bytes().first() == Some(&b'-'))
            || s.lines().map(len).sum::<f32>() > 15.0
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

    #[test]
    fn test_fit_on_one_line() {
        let l = FitOnOneLine;
        assert!(!l.lint("Length of\n16 chr"));
        assert!(l.lint("This line\n has more than 16"));
        assert!(l.lint("- short\n- dialog"));
    }
}
