use super::Lint;
use regex::Regex;

macro_rules! regex_global_lint {
    ($struct_name:ident, $code:tt, $desc:tt, $re:tt) => {
        #[derive(Debug, Default)]
        pub struct $struct_name;

        impl Lint for $struct_name {
            fn code(&self) -> &'static str {
                $code
            }

            fn message(&self) -> &'static str {
                $desc
            }

            fn lint(&self, s: &str) -> bool {
                let re = Regex::new($re).unwrap();
                !re.is_match(s)
            }
        }
    };
}

macro_rules! regex_line_lint {
    ($struct_name:ident, $code:tt, $desc:tt, $re:tt) => {
        #[derive(Debug, Default)]
        pub struct $struct_name;

        impl Lint for $struct_name {
            fn code(&self) -> &'static str {
                $code
            }

            fn message(&self) -> &'static str {
                $desc
            }

            fn lint(&self, s: &str) -> bool {
                let re = Regex::new($re).unwrap();
                !s.lines().any(|l| re.is_match(l))
            }
        }
    };
}

regex_line_lint!(DoubleSpace, "LXX-001", "No double space allowed", r"\s{2,}");
regex_line_lint!(
    TrailingSpace,
    "LXX-002",
    "No trailing space allowed",
    r"\s$"
);
regex_line_lint!(LeadingSpace, "LXX-003", "No leading space allowed", r"^\s");
regex_line_lint!(EmptyLines, "LXX-004", "No empty lines allowed", r"^\s*$");
regex_global_lint!(
    HyphenTwoLines,
    "NX05-001-1",
    "Hyphen should be used only when two speakers",
    r"^-.*\n[^-]|^[^-].*\n-"
);
regex_line_lint!(
    HyphenWithoutSpace,
    "NX05-001-2",
    "Use an English hyphen without a space",
    r"^-(\s|$)"
);
regex_line_lint!(
    NoFullWidthNumber,
    "NX11-002",
    "No full-width number allowed",
    r"[\u{ff10}-\u{ff19}]"
);
regex_line_lint!(
    NoCommaOrPeriod,
    "NX12-001",
    "No comma or period allowed",
    r"[.,]"
);
regex_line_lint!(
    NoU22EFEllipsis,
    "NX12-008",
    "No U+22EF ellipsis allowed",
    r"\u{22EF}"
);
regex_line_lint!(
    NoDuplicatedPunctuation,
    "NX12-009",
    "No duplicated punctuation allowed",
    r"[.,?!？…！]{2,}"
);

regex_global_lint!(
    QuotesMismatchOpeningOnly,
    "LXX-006-001",
    "Opening quote without closing quote",
    r"“[^”]*$"
);
regex_global_lint!(
    QuotesMismatchClosingOnly,
    "LXX-006-002",
    "closing quote without opening quote",
    r"^[^“]*”"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_space() {
        let l = DoubleSpace::default();
        assert!(l.lint("This is a test"));
        assert!(!l.lint("This  is a test"));
    }

    #[test]
    fn test_trailing_space() {
        let l = TrailingSpace::default();
        assert!(l.lint("This is a test"));
        assert!(!l.lint("This is a test "));
    }

    #[test]
    fn test_leading_space() {
        let l = LeadingSpace::default();
        assert!(l.lint("This is a test"));
        assert!(!l.lint(" This is a test"));
    }

    #[test]
    fn test_empty_lines() {
        let l = EmptyLines::default();
        assert!(l.lint("This is a test"));
        assert!(!l.lint("This is a test\n\nThis is a test"));
    }

    #[test]
    fn test_hyphen_two_lines() {
        let l = HyphenTwoLines::default();
        assert!(l.lint("This is a test"));
        assert!(l.lint("- This is a test\n- This is a test"));
        assert!(!l.lint("- This is a test\nThis is a test"));
        assert!(!l.lint("This is a test\n- This is a test"));
    }

    #[test]
    fn test_hyphen_without_space() {
        let l = HyphenWithoutSpace::default();
        assert!(l.lint("This is a test"));
        assert!(l.lint("-This is a test"));
        assert!(!l.lint("- This is a test"));
        assert!(!l.lint("-"));
    }

    #[test]
    fn test_no_duplicated_punctuation() {
        let l = NoDuplicatedPunctuation::default();
        assert!(l.lint("This is a test"));
        assert!(l.lint("This is a test!"));
        assert!(!l.lint("This is a test!!"));
        assert!(!l.lint("This is a test？？"));
        assert!(!l.lint("This is a test！？"));
        assert!(!l.lint("This is a test！！"));
        assert!(!l.lint("This is a test……"));
    }

    #[test]
    fn test_no_full_width_number() {
        let l = NoFullWidthNumber::default();
        assert!(l.lint("This is a test"));
        assert!(l.lint("This is a test 1"));
        assert!(!l.lint("This is a test １"));
    }

    #[test]
    fn test_no_comma_or_period() {
        let l = NoCommaOrPeriod::default();
        assert!(l.lint("This is a test"));
        assert!(!l.lint("This is a test,"));
        assert!(!l.lint("This is a test."));
        assert!(!l.lint("This is a test,."));
    }

    #[test]
    fn test_no_u22ef_ellipsis() {
        let l = NoU22EFEllipsis::default();
        assert!(l.lint("This is a test…"));
        assert!(!l.lint("This is a test⋯"));
    }

    #[test]
    fn test_quotes_mismatch_opening_only() {
        let l = QuotesMismatchOpeningOnly::default();
        assert!(l.lint("This is a test"));
        assert!(!l.lint("“This is a test"));

        assert!(l.lint("This is a test “”"));
    }

    #[test]
    fn test_quotes_mismatch_closing_only() {
        let l = QuotesMismatchClosingOnly::default();
        assert!(l.lint("This is a test"));
        assert!(!l.lint("This is a test”"));
        assert!(l.lint("This is a test “”"));
    }
}
