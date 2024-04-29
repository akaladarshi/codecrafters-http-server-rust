use std::io;
use std::io::Write;

use nom::AsBytes;

use crate::content::Content;

pub struct Body {
    content_type: Content,
    content: Vec<u8>,
}

impl Body {
    pub fn new(content_type: &str, data: Vec<u8>) -> Body {
        Body{
            content_type: Content::get_content(content_type),
            content: data,
        }
    }

    pub fn empty() -> Self {
        Body{
            content_type: Default::default(),
            content:  Default::default(),
        }
    }

    pub fn get_content_type(&self) -> &str {
        self.content_type.string()
    }

    pub fn get_content(&self) -> Vec<u8> {
        self.content.clone()
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<usize, io::Error> {
        writer.write(self.content.as_bytes())
    }

}
