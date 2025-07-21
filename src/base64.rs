const BASE64_INPUT_WORDS: usize = 3;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

// The following masks are applied to each of the three input bytes twice, the
// first time as is and the second time inverted.
//
//     0xfc -!> 0x03
//     0xf0 -!> 0x0f
//     0xc0 -!> 0x3f
//
const INPUT_0: u8 = 0xfc; // 0b1111_1100
const INPUT_1: u8 = 0xf0; // 0b1111_0000
const INPUT_2: u8 = 0xc0; // 0b1100_0000

/// Base64 encoding implementation.
///
/// By evaluating in 24-bit steps the input slice, transform the three 8-bit
/// words in four, zero-padded 6-bit words, until the input is fully consumed.
///
/// The 24-bit evaluation leads to three possible scenarios:
///
/// # Full 3-bytes input
/// ```txt
/// input  -> 1111 2222 | 3333 4444 | 5555 6666
/// output -> 0011 1122 | 0022 3333 | 0044 4455 | 0055 6666
/// ```
///
/// # 2-bytes input, last byte is empty
/// ```txt
/// input  -> 1111 2222 | 3333 4444 | 0000 0000
/// output -> 0011 1122 | 0022 3333 | 0044 4400 | 0000 0000
/// ```
///
/// # 1-byte input, last 2 bytes are empty
/// ```txt
/// input  -> 1111 2222 | 0000 0000 | 0000 0000
/// output -> 0011 1122 | 0022 0000 | 0000 0000 | 0000 0000
/// ```
///
/// When there are empty bytes, the trailing `0x0` bytes are padded with `=`
/// character.
fn encode_bytes(input: &[u8], output: &mut [u8]) {
    let mut i: usize = 0;
    let mut chunks = input.chunks_exact(BASE64_INPUT_WORDS);

    for chunk in &mut chunks {
        output[i] = CHARSET[((chunk[0] & INPUT_0) >> 2) as usize];
        output[i + 1] =
            CHARSET[(((chunk[0] & !INPUT_0) << 4) | ((chunk[1] & INPUT_1) >> 4)) as usize];
        output[i + 2] =
            CHARSET[(((chunk[1] & !INPUT_1) << 2) | ((chunk[2] & INPUT_2) >> 6)) as usize];
        output[i + 3] = CHARSET[(chunk[2] & !INPUT_2) as usize];
        i += 4;
    }

    let padding = CHARSET[CHARSET.len() - 1];
    match chunks.remainder() {
        [b0, b2] => {
            output[i] = CHARSET[((b0 & INPUT_0) >> 2) as usize];
            output[i + 1] = CHARSET[(((b0 & !INPUT_0) << 4) | ((b2 & INPUT_1) >> 4)) as usize];
            output[i + 2] = CHARSET[((b2 & !INPUT_1) << 2) as usize];
            output[i + 3] = padding;
        }
        [b0] => {
            output[i] = CHARSET[((b0 & INPUT_0) >> 2) as usize];
            output[i + 1] = CHARSET[((b0 & !INPUT_0) << 4) as usize];
            output[i + 2] = padding;
            output[i + 3] = padding;
        }
        [] => {}
        _ => unreachable!("base64 chunks are visited in groups of 3 bytes"),
    }
}

pub fn encode(input: &[u8]) -> String {
    let output_len = (input.len() + 2) / 3 * 4;
    let mut buf = vec![0u8; output_len];
    encode_bytes(input, &mut buf);
    String::from_utf8(buf).expect("base64 charset is UTF-8 safe")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_cover_rfc_tests() {
        let string = "";
        let encoded = encode(string.as_bytes());
        assert_eq!(encoded, "");

        let string = "f";
        let encoded = encode(string.as_bytes());
        assert_eq!(encoded, "Zg==");

        let string = "fo";
        let encoded = encode(string.as_bytes());
        assert_eq!(encoded, "Zm8=");

        let string = "foo";
        let encoded = encode(string.as_bytes());
        assert_eq!(encoded, "Zm9v");

        let string = "foob";
        let encoded = encode(string.as_bytes());
        assert_eq!(encoded, "Zm9vYg==");

        let string = "fooba";
        let encoded = encode(string.as_bytes());
        assert_eq!(encoded, "Zm9vYmE=");

        let string = "foobar";
        let encoded = encode(string.as_bytes());
        assert_eq!(encoded, "Zm9vYmFy");
    }
}
