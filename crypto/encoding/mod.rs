#![allow(dead_code)]
pub(super) mod base64;

pub trait Encoding {
    type Success;
    type Error;

    fn encode(&self, input: impl AsRef<[u8]>) -> Result<Self::Success, Self::Error>;

    fn decode(&self, input: impl AsRef<[u8]>) -> Result<Self::Success, Self::Error>;
}

/// Support Different Engine that implement the Encoding Trait
///
/// Return The Engine Success Value and Error Respectively
pub fn encode<T>(enc: &T, input: impl AsRef<[u8]>) -> Result<T::Success, T::Error>
where
    T: Encoding,
{
    enc.encode(input)
}

/// Support Different Engine that implement the Encoding Trait
///
/// Return The Engine Success Value and Error Respectively
pub fn decode<T>(dec: &T, input: impl AsRef<[u8]>) -> Result<T::Success, T::Error>
where
    T: Encoding,
{
    dec.decode(input)
}

#[cfg(test)]
mod test {
    use super::{base64::*, Encoding};
    use anyhow::Result;

    /// Encode and Decode the value with provided engine
    ///
    /// # Errors
    ///
    /// This function will return an error if encoding and decoding failed.
    fn encode_and_decode_handler<T, A>(engine: &T, value: A, msg: &str) -> Result<()>
    where
        T: Encoding<Success = String, Error = Error>,
        A: AsRef<[u8]> + Copy,
    {
        use super::{decode, encode};
        let enc_content = encode(engine, value)?;
        println!("{} - {:?}", msg, enc_content);
        let dec_content = decode(engine, enc_content)?;
        println!("{} - {:?}", msg, dec_content);

        // Compare value with decoded content after encoding and decoding
        assert_eq!(std::str::from_utf8(value.as_ref())?, dec_content);
        Ok(())
    }

    #[test]
    fn test_base_64_standard() -> Result<()> {
        encode_and_decode_handler(&B64::<Standard>::new(), "ABCDGETAJHE", "BASE64 - STANDARD")
    }

    #[test]
    fn test_base_64_standard_no_pad() -> Result<()> {
        encode_and_decode_handler(
            &B64::<StandardNopad>::new(),
            "ABCDGETAJHE",
            "BASE64 - STANDARD NOPAD",
        )
    }

    #[test]
    fn test_base_64_url_safe() -> Result<()> {
        encode_and_decode_handler(&B64::<UrlSafe>::new(), "ABCDGETAJHE", "BASE64 - URLSAFE")
    }

    #[test]
    fn test_base_64_url_safe_no_pad() -> Result<()> {
        encode_and_decode_handler(
            &B64::<UrlSafeNopad>::new(),
            "ABCDGETAJHE",
            "BASE64 - URLSAFE NOPAD",
        )
    }
}
