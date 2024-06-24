use crate::constants::GZIP;

pub enum EncodingScheme {
    Gzip,
}

impl EncodingScheme {
    pub fn get_encoding_scheme(scheme: &str) -> Option<EncodingScheme> {
        match scheme {
            GZIP => Option::from(EncodingScheme::Gzip),
            _ =>  None,
        }
    }

    pub fn string(&self) -> &str {
        match *self {
            EncodingScheme::Gzip => GZIP,
        }
    }
}