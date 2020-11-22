
use super::types::{FlexStyle, StyleDeclaration, ThemeStyle};

pub fn is_layout_style(declaration: &StyleDeclaration) -> Option<&FlexStyle> {
    match declaration {
        &StyleDeclaration::Layout(ref v) => Some(v),
        _ => None
    }
}

pub fn is_theme_style(declaration: &StyleDeclaration) -> Option<&ThemeStyle> {
    match declaration {
        &StyleDeclaration::Theme(ref v) => Some(v),
        _ => None
    }
}
