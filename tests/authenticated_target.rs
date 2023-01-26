use camo_rs::authenticated_target::*;

// The encoded values were generated and validated using the original Camo
// to make sure that this implementation is frontend-compatible.
const VALID_KEY: &[u8] = "example".as_bytes();
const VALID_TARGET: &str = "http://example.com/a.webp";
const VALID_ENCODED_DIGEST: &str = "78d0f8623f9ef3f0b58bf1adac2b3f02154af36d";
const VALID_ENCODED_TARGET: &str = "687474703a2f2f6578616d706c652e636f6d2f612e77656270";

#[test]
fn from_encoded_strings_fails_gracefully_with_empty_key() {
    let result =
        AuthenticatedTarget::from_encoded_strings(&[], VALID_ENCODED_DIGEST, VALID_ENCODED_TARGET);

    assert!(result.is_err());
}

#[test]
fn from_encoded_strings_fails_gracefully_with_junk_digest() {
    let result = AuthenticatedTarget::from_encoded_strings(VALID_KEY, "abz", VALID_ENCODED_TARGET);

    assert!(result.is_err());
}

#[test]
fn from_encoded_strings_fails_gracefully_with_junk_target() {
    let result = AuthenticatedTarget::from_encoded_strings(VALID_KEY, VALID_ENCODED_DIGEST, "abz");

    assert!(result.is_err());
}

#[test]
fn from_encoded_strings_rejects_empty_digest() {
    let result = AuthenticatedTarget::from_encoded_strings(VALID_KEY, "", VALID_ENCODED_DIGEST);

    assert!(result.is_err());
}

#[test]
fn from_encoded_strings_accepts_valid_data() {
    let result = AuthenticatedTarget::from_encoded_strings(
        VALID_KEY,
        VALID_ENCODED_DIGEST,
        VALID_ENCODED_TARGET,
    );
    assert!(result.is_ok());
}

#[test]
fn validate_rejects_empty_digest() {
    let result = AuthenticatedTarget::from_encoded_strings(VALID_KEY, "", VALID_ENCODED_TARGET)
        .unwrap()
        .validated_target_url();

    assert!(result.is_err());
}

#[test]
fn validate_rejects_invalid_digest() {
    let result = AuthenticatedTarget::from_encoded_strings(
        VALID_KEY,
        VALID_ENCODED_DIGEST,
        "687474703a2f2f6e6f742e6578616d706c652e636f6d2f622e77656270",
    )
    .unwrap()
    .validated_target_url();

    assert!(result.is_err());
}

#[test]
fn validate_accepts_valid_data() {
    let result = AuthenticatedTarget::from_encoded_strings(
        VALID_KEY,
        VALID_ENCODED_DIGEST,
        VALID_ENCODED_TARGET,
    )
    .unwrap()
    .validated_target_url();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), VALID_TARGET);
}

#[test]
fn from_target_generates_valid_url() {
    let expected = format!("{VALID_ENCODED_DIGEST}/{VALID_ENCODED_TARGET}");
    let target = AuthenticatedTarget::from_target(VALID_KEY, VALID_TARGET);

    assert_eq!(target.encoded_full_path(), expected);
}
