mod non_regex;
mod regex;

use enum_dispatch::enum_dispatch;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[enum_dispatch]
pub trait Lint {
    fn code(&self) -> &'static str;
    fn message(&self) -> &'static str;
    fn lint(&self, s: &str) -> bool;
}

#[enum_dispatch(Lint)]
#[derive(Debug, EnumIter)]
pub enum Lints {
    Spaces(regex::DoubleSpace),
    TrailingSpace(regex::TrailingSpace),
    LeadingSpace(regex::LeadingSpace),
    EmptyLines(regex::EmptyLines),
    HyphenTwoLines(regex::HyphenTwoLines),
    HyphenWithoutSpace(regex::HyphenWithoutSpace),
    CharMaxPerLine(non_regex::CharMaxPerLine),
    LineCount(non_regex::LineCount),
    NoDuplicatedPunctuation(regex::NoDuplicatedPunctuation),
    NoFullWidthNumber(regex::NoFullWidthNumber),
    NoCommaOrPeriod(regex::NoCommaOrPeriod),
    NoU22EFEllipsis(regex::NoU22EFEllipsis),
}

impl Lints {
    pub fn check_on(s: &str) -> Vec<Lints> {
        Lints::iter().filter(|l| !l.lint(s)).collect()
    }
}
