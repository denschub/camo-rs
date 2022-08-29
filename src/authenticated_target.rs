//! Processes and validates encoded target parameters from a Camo URL, and can
//! be used to generate Camo URLs as well.

use hmac::{Hmac, Mac};
use sha1::Sha1;

use crate::errors::{AuthParsingError, AuthValidationError};

/// The machinery to parse and build Authenticated Target URLs.
pub struct AuthenticatedTarget {
    key: Vec<u8>,
    digest: Vec<u8>,
    target: String,
}

impl AuthenticatedTarget {
    /// Takes a known key and a target URL, useful for converting known plain
    /// data into a Camo URL.
    pub fn from_target(key: &[u8], target: &str) -> Self {
        let digest = Self::calculate_hmac(key, target.as_bytes());

        Self {
            key: key.to_vec(),
            digest,
            target: target.to_owned(),
        }
    }

    /// Takes a known key, and a user-provided Digest and encoded Target URL,
    /// usually from a request. While this does parse the data, validation
    /// is happening later.
    pub fn from_encoded_strings(
        key: &[u8],
        digest: &str,
        target: &str,
    ) -> Result<Self, AuthParsingError> {
        if key.is_empty() {
            return Err(AuthParsingError::EmptyKeyError);
        }

        let digest = hex::decode(digest).map_err(AuthParsingError::DigestEncodingError)?;

        let target = hex::decode(target).map_err(AuthParsingError::TargetEncodingError)?;
        let target = String::from_utf8(target).map_err(AuthParsingError::TargetNotUtf8)?;

        Ok(Self {
            key: key.to_vec(),
            digest,
            target,
        })
    }

    /// Tries to validate the Target URL by calculating the HMAC and comparing
    /// it with the user-provided value. Returns the plain Target URL if it is
    /// valid, and a `AuthValidationError` otherwise.
    pub fn validated_target_url(&self) -> Result<String, AuthValidationError> {
        let target = self.target.as_bytes();
        let expected = Self::calculate_hmac(&self.key, target);

        if self.digest == expected {
            Ok(self.target.to_owned())
        } else {
            Err(AuthValidationError::HmacInvalid)
        }
    }

    /// Returns the hex-encoded Digest part (the first URL segment).
    pub fn encoded_digest(&self) -> String {
        hex::encode(self.digest.as_slice())
    }

    /// Returns the hex-encoded Target URL part (the second URL segment).
    pub fn encoded_target_url(&self) -> String {
        hex::encode(self.target.as_bytes())
    }

    /// Returns a full Camo URL, without a leading slash.
    pub fn encoded_full_path(&self) -> String {
        format!("{}/{}", self.encoded_digest(), self.encoded_target_url())
    }

    /// Calculates the HMAC from a key and a target.
    fn calculate_hmac(key: &[u8], target: &[u8]) -> Vec<u8> {
        let mut mac = Hmac::<Sha1>::new_from_slice(key).expect("HMAC can take key of any size");
        mac.update(target);
        mac.finalize().into_bytes().to_vec()
    }
}
