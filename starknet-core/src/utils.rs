use crate::types::FieldElement;

use sha3::{Digest, Keccak256};
use thiserror::Error;

const DEFAULT_ENTRY_POINT_NAME: &str = "__default__";
const DEFAULT_L1_ENTRY_POINT_NAME: &str = "__l1_default__";

#[derive(Debug, Error)]
#[error("the provided name contains non-ASCII characters: {name}")]
pub struct NonAsciiNameError<'a> {
    pub name: &'a str,
}

#[derive(Debug, Error)]
pub enum CairoShortStringToFeltError {
    #[error("Cairo string can only contain ASCII characters")]
    NonAsciiCharacter,
    #[error("short string exceeds maximum length of 31 characters")]
    StringTooLong,
}

/// A variant of eth-keccak that computes a value that fits in a StarkNet field element.
pub fn starknet_keccak(data: &[u8]) -> FieldElement {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let mut hash = hasher.finalize();

    // Remove the first 6 bits
    hash[0] &= 0b00000011;

    // Because we know hash is always 32 bytes
    FieldElement::from_bytes_be(unsafe { &*(hash[..].as_ptr() as *const [u8; 32]) }).unwrap()
}

pub fn get_selector_from_name(func_name: &str) -> Result<FieldElement, NonAsciiNameError> {
    if func_name == DEFAULT_ENTRY_POINT_NAME || func_name == DEFAULT_L1_ENTRY_POINT_NAME {
        Ok(FieldElement::ZERO)
    } else {
        let name_bytes = func_name.as_bytes();
        if name_bytes.is_ascii() {
            Ok(starknet_keccak(name_bytes))
        } else {
            Err(NonAsciiNameError { name: func_name })
        }
    }
}

/// Converts Cairo short string to [FieldElement].
pub fn cairo_short_string_to_felt(str: &str) -> Result<FieldElement, CairoShortStringToFeltError> {
    if !str.is_ascii() {
        return Err(CairoShortStringToFeltError::NonAsciiCharacter);
    }
    if str.len() > 31 {
        return Err(CairoShortStringToFeltError::StringTooLong);
    }

    let ascii_bytes = str.as_bytes();

    let mut buffer = [0u8; 32];
    buffer[(32 - ascii_bytes.len())..].copy_from_slice(ascii_bytes);

    // The conversion will never fail
    Ok(FieldElement::from_bytes_be(&buffer).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_starknet_keccak() {
        // Generated from `cairo-lang`
        let data = b"execute";
        let expected_hash = FieldElement::from_hex_be(
            "0240060cdb34fcc260f41eac7474ee1d7c80b7e3607daff9ac67c7ea2ebb1c44",
        )
        .unwrap();

        let hash = starknet_keccak(data);

        assert_eq!(hash, expected_hash);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_selector_from_name() {
        // Generated from `cairo-lang`
        let func_name = "execute";
        let expected_selector = FieldElement::from_hex_be(
            "0240060cdb34fcc260f41eac7474ee1d7c80b7e3607daff9ac67c7ea2ebb1c44",
        )
        .unwrap();

        let selector = get_selector_from_name(func_name).unwrap();

        assert_eq!(selector, expected_selector);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_default_selector() {
        let default_selector = FieldElement::from_hex_be(
            "0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap();

        assert_eq!(
            get_selector_from_name("__default__").unwrap(),
            default_selector
        );
        assert_eq!(
            get_selector_from_name("__l1_default__").unwrap(),
            default_selector
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_get_selector_from_non_ascii_name() {
        let func_name = "🦀";

        match get_selector_from_name(func_name) {
            Err(_) => {}
            _ => panic!("Should throw error on non-ASCII name"),
        };
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_cairo_short_string_to_felt() {
        let data = [
            (
                "abcdefghijklmnopqrstuvwxyz",
                "156490583352162063278528710879425690470022892627113539022649722",
            ),
            (
                "1234567890123456789012345678901",
                "86921973946889608444641514252360676678984087116218318142845213717418291249",
            ),
        ];

        for (str, felt_dec) in data.into_iter() {
            assert_eq!(
                cairo_short_string_to_felt(str).unwrap(),
                FieldElement::from_dec_str(felt_dec).unwrap()
            );
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_cairo_short_string_to_felt_too_long() {
        assert!(matches!(
            cairo_short_string_to_felt("12345678901234567890123456789012"),
            Err(CairoShortStringToFeltError::StringTooLong)
        ));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_cairo_short_string_to_felt_non_ascii() {
        assert!(matches!(
            cairo_short_string_to_felt("🦀"),
            Err(CairoShortStringToFeltError::NonAsciiCharacter)
        ));
    }
}
