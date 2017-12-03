//! Common data types that are used throughout the library.

use error::Result;

/// All the data types defined by the EBML standard.
pub mod types {
    pub type Binary = Vec<u8>;
    pub type UnsignedInt = u64;
    pub type SignedInt = i64;
    pub type Float = f64;
    pub type Utf8 = String;

    pub type ElementId = UnsignedInt;
    pub type ElementSize = usize;
}

pub struct ElementContent(Vec<u8>);

impl ElementContent {
    /// Create a new Element Content object.
    pub fn new(data: Vec<u8>) -> ElementContent {
        ElementContent(data)
    }

    /// Interpret the element content as raw binary data. Consumes `self`.
    pub fn into_binary(self) -> Vec<u8> {
        self.0
    }

    /// Interpret the element content as an unsigned integer. Consumes `self`.
    pub fn into_uint(self) -> types::UnsignedInt {
        let buf = self.0;
        let mut value = 0 as u64;

        for i in 0..buf.len() {
            value |= (buf[buf.len() - i - 1] as u64) << i * 8;
        }

        value
    }

    /// Interpret the element content as a signed integer. Consumes `self`.
    pub fn into_int(self) -> types::SignedInt {
        let buf = self.0;
        let mut value = 0 as i64;

        for i in 0..buf.len() {
            value |= (buf[buf.len() - i - 1] as i64) << i * 8;
        }

        value
    }

    /// Interpret the element content as a floating point number. Consumes `self`.
    pub fn into_float(self) -> types::Float {
        f64::from_bits(self.into_uint())
    }

    /// Interpret the element content as an UTF-8 string. Can return an error if the data in not
    /// valid UTF-8. Consumes `self`.
    pub fn into_utf8(self) -> Result<types::Utf8> {
        Ok(String::from_utf8(self.into_binary())?)
    }
}