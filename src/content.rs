use crate::constants::CONTENT_TYPE_TEXT;

pub enum Content {
    TextPlain,
    Default
}

impl Content {
    pub fn get_content(c: &str) -> Content {
        match c {
            CONTENT_TYPE_TEXT => Content::TextPlain,
            _ => Content::Default
        }
    }

    pub fn string(&self) -> &str {
        match *self {
            Content::TextPlain => CONTENT_TYPE_TEXT,
            Content::Default => "",
        }
    }
}

impl Default for Content {
    fn default() -> Self {
        Content::Default
    }
}