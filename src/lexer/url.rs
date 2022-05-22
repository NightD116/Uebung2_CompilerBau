use logos::{Lexer, Logos};
use std::{fmt::{Display, Formatter}};
use regex::Regex;
/// Tuple struct for link URLs
#[derive(Debug, PartialEq)]
pub struct LinkUrl(String);

/// Implement Display for printing
impl Display for LinkUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Tuple struct for link texts
#[derive(Debug, PartialEq)]
pub struct LinkText(String);

/// Implement Display for printing
impl Display for LinkText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Token enum for capturing of link URLs and Texts
#[derive(Logos, Debug, PartialEq)]
pub enum URLToken {
    // <a[^>]*href=\"[^\"]*\"[^>]*>[^<]*</a[ \r\t\n]*>
    #[regex("<a[^>]*href=[^>]*>[^<]*</a[\\s\n]*>", extract_link_info)]
    Link((LinkUrl, LinkText)),

    #[regex("<[^>]*>", logos::skip)]
    #[regex("[^<]*", logos::skip)]
    Ignored,

    // Catch any error
    #[error]
    #[regex(r" \n\r\t\f", logos::skip)]
    Error,
}

/// Extracts the URL and text from a string that matched a Link token
fn extract_link_info(lex: &mut Lexer<URLToken>) -> (LinkUrl, LinkText) {
    let slice = lex.slice();
    let regex = Regex::new("(?:href=\")([^\"]*)(?:\")(?:[^>]*>)([^<]*)(?:<)").unwrap();
    let capture = regex.captures(slice).unwrap();
    let url = capture.get(1).unwrap().as_str();
    let text = capture.get(2).unwrap().as_str();
    (LinkUrl(url.to_string()),LinkText(text.to_string()))
}
