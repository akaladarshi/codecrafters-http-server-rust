use crate::constants::{CONTENT_TYPE_OCTET, CONTENT_TYPE_TEXT};

pub enum Content {
    Text,
    Octet,
    Default,
}

impl Content {
    pub fn get_content(c: &str) -> Content {
        match c {
            CONTENT_TYPE_TEXT => Content::Text,
            CONTENT_TYPE_OCTET=> Content::Octet,
            _ => Content::Default
        }
    }

    pub fn string(&self) -> &str {
        match *self {
            Content::Text => CONTENT_TYPE_TEXT,
            Content::Octet => CONTENT_TYPE_OCTET,
            Content::Default => "",
        }
    }
}

impl Default for Content {
    fn default() -> Self {
        Content::Default
    }
}