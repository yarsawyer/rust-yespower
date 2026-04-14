#![cfg_attr(not(feature = "std"), no_std)]

use core::ffi::{c_char, c_int};
use core::fmt;

/// Serialized pure Tidecoin block header length.
pub const TIDECOIN_HEADER_LEN: usize = 80;

/// Tidecoin yespower output length.
pub const TIDECOIN_HASH_LEN: usize = 32;

unsafe extern "C" {
    fn yespower_hash(input: *const c_char, output: *mut c_char) -> c_int;
}

/// Error returned when the yespower backend rejects the input or fails internally.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Tidecoin yespower hash failed")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// Computes Tidecoin yespower over a serialized pure 80-byte block header.
///
/// Tidecoin uses yespower 1.0 with `N = 2048`, `r = 8`, and no personalization
/// before AuxPoW activation.
pub fn tidecoin_hash(input: &[u8; TIDECOIN_HEADER_LEN]) -> Result<[u8; TIDECOIN_HASH_LEN], Error> {
    let mut output = [0u8; TIDECOIN_HASH_LEN];
    // SAFETY: the C function reads exactly 80 bytes from `input` and writes exactly 32 bytes to
    // `output` for the fixed Tidecoin wrapper. Both pointers are valid for those lengths and do
    // not alias mutably.
    let result = unsafe {
        yespower_hash(
            input.as_ptr().cast::<c_char>(),
            output.as_mut_ptr().cast::<c_char>(),
        )
    };

    if result == 0 {
        Ok(output)
    } else {
        Err(Error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let input_hex = "0000002009f42768de3cfb4e58fc56368c1477f87f60e248d7130df3fb8acd7f6208b83a72f90dd3ad8fe06c7f70d73f256f1e07185dcc217a58b9517c699226ac0297d2ad60ba61b62a021d9b7700f0";
        let expected_output_hex =
            "9d90c21b5a0bb9566d2999c5d703d7327ee3ac97c020d387aa2dfd0700000000";

        let input_bytes: [u8; 80] = hex::decode(input_hex)
            .expect("Decoding failed")
            .try_into()
            .expect("Incorrect input length");
        let expected_output_bytes: [u8; 32] = hex::decode(expected_output_hex)
            .expect("Decoding failed")
            .try_into()
            .expect("Incorrect output length");

        assert_eq!(tidecoin_hash(&input_bytes).unwrap(), expected_output_bytes);
    }
}
