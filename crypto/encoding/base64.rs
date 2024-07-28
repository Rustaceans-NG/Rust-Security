use super::Encoding;
use base64::{
    engine::{
        general_purpose::{NO_PAD, PAD},
        GeneralPurpose,
    },
    DecodeError, Engine as _,
};
use derive_new::new;
use std::marker::PhantomData;

/// ```no_rust
/// Base64
///
/// Encode and Decode bytes using base64 encoding and it support different algorithm and open for addition
///
/// Algorithm
/// - Standard Padding
/// - Standard No Padding
/// - Url Safe Padding
/// - Url Safe No Padding
/// ```
#[derive(new)]
pub struct B64<T = UrlSafe>(PhantomData<T>);

const STANDARD: GeneralPurpose = GeneralPurpose::new(&base64::alphabet::STANDARD, PAD);
const STANDARD_NO_PAD: GeneralPurpose = GeneralPurpose::new(&base64::alphabet::STANDARD, NO_PAD);
const URL_SAFE: GeneralPurpose = GeneralPurpose::new(&base64::alphabet::URL_SAFE, PAD);
const URL_SAFE_NO_PAD: GeneralPurpose = GeneralPurpose::new(&base64::alphabet::URL_SAFE, NO_PAD);

// These are used to enforced the standard we want
macro_rules! impl_encoding {
    ($name:ident, $alg:expr) => {
        pub struct $name;
        impl Encoding for B64<$name> {
            type Success = String;
            type Error = Error;

            fn encode(&self, input: impl AsRef<[u8]>) -> Result<Self::Success, Self::Error> {
                Ok($alg.encode(input))
            }

            fn decode(&self, input: impl AsRef<[u8]>) -> Result<Self::Success, Self::Error> {
                $alg.decode(input)
                    .map(String::from_utf8)?
                    .map_err(|_| Error(String::from("Failed to convert from bytes to string!")))
            }
        }
    };
}
// Supported Algorithm
impl_encoding!(UrlSafe, URL_SAFE);
impl_encoding!(Standard, STANDARD);
impl_encoding!(UrlSafeNopad, URL_SAFE_NO_PAD);
impl_encoding!(StandardNopad, STANDARD_NO_PAD);

#[derive(Debug)]
pub struct Error(pub String);

impl From<DecodeError> for Error {
    fn from(value: DecodeError) -> Self {
        match value {
            DecodeError::InvalidByte(offset, bytes) => Error(format!(
                "Invalid token byte at offset: {} bytes = {}",
                offset, bytes
            )),
            DecodeError::InvalidLength(len) => Error(format!(
                "The length of the token is invalid length: {}",
                len
            )),
            DecodeError::InvalidLastSymbol(offset, byte) => Error(format!(
                "This token failed encoding due to invalid last symbols offset: {} bytes = {}",
                offset, byte
            )),
            DecodeError::InvalidPadding => Error(String::from(
                "This token failed encoding to due to invalid padding",
            )),
        }
    }
}
impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
