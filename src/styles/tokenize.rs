
use djed_self_tokenize_trait::{ToCustomTokens, Tokens};

use super::types::{FontName, GenericFontName, SpecificFontName, StyleSelector, Stylesheet};

impl ToCustomTokens for Stylesheet {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        let mut inner_tokens = Tokens::new();
        self.rules.to_custom_tokens(&mut inner_tokens);
        tokens.append(quote! { Stylesheet::from(#inner_tokens) });
    }
}

impl ToCustomTokens for StyleSelector {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        let string_ref: &str = self.as_ref();
        tokens.append(quote! { StyleSelector::from(#string_ref) });
    }
}

impl ToCustomTokens for FontName {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        match self {
            &FontName::Generic(ref n) => {
                tokens.append(quote! { FontName::Generic(#n) });
            }
            &FontName::Specific(ref n) if n.as_ref().eq_ignore_ascii_case("serif") => {
                FontName::Generic(GenericFontName::Serif).to_custom_tokens(tokens);
            }
            &FontName::Specific(ref n) if n.as_ref().eq_ignore_ascii_case("sans-serif") => {
                FontName::Generic(GenericFontName::SansSerif).to_custom_tokens(tokens);
            }
            &FontName::Specific(ref n) if n.as_ref().eq_ignore_ascii_case("monospace") => {
                FontName::Generic(GenericFontName::Monospace).to_custom_tokens(tokens);
            }
            &FontName::Specific(ref n) if n.as_ref().eq_ignore_ascii_case("cursive") => {
                FontName::Generic(GenericFontName::Cursive).to_custom_tokens(tokens);
            }
            &FontName::Specific(ref n) if n.as_ref().eq_ignore_ascii_case("fantasy") => {
                FontName::Generic(GenericFontName::Fantasy).to_custom_tokens(tokens);
            }
            &FontName::Specific(ref n) if n.as_ref().eq_ignore_ascii_case("system-ui") => {
                FontName::Generic(GenericFontName::SystemUI).to_custom_tokens(tokens);
            }
            &FontName::Specific(ref n) => {
                tokens.append(quote! { FontName::Specific(#n) });
            }
        }
    }
}

impl ToCustomTokens for SpecificFontName {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        let string_ref: &str = self.as_ref();
        tokens.append(quote! { SpecificFontName::from(#string_ref) });
    }
}
