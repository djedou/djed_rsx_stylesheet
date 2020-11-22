
use std::borrow::Cow;

use super::types::{InlineRules, SpecificFontName, StyleSelector, Stylesheet};

impl From<InlineRules> for Stylesheet {
    fn from(rules: InlineRules) -> Self {
        Stylesheet { rules }
    }
}

impl From<&'static str> for StyleSelector {
    fn from(string: &'static str) -> Self {
        StyleSelector(Cow::from(string))
    }
}

impl From<String> for StyleSelector {
    fn from(string: String) -> Self {
        StyleSelector(Cow::from(string))
    }
}

impl From<&'static str> for SpecificFontName {
    fn from(string: &'static str) -> Self {
        SpecificFontName(Cow::from(string))
    }
}

impl From<String> for SpecificFontName {
    fn from(string: String) -> Self {
        SpecificFontName(Cow::from(string))
    }
}
