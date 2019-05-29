// Lightly modified from https://github.com/v8/v8/blob/master/src/third_party/utf8-decoder/utf8-decoder.h
// This is the same UTF-8 Decoder that I wrote for v8, based on Björn Höhrmann's design.
// http://bjoern.hoehrmann.de/utf-8/decoder/dfa/

  // This first table maps bytes to a transition.
static TRANSITIONS: [u8; 256] = [
    0,  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 00-0F
    0,  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 10-1F
    0,  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 20-2F
    0,  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 30-3F
    0,  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 40-4F
    0,  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 50-5F
    0,  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 60-6F
    0,  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 70-7F
    1,  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 80-8F
    2,  2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // 90-9F
    3,  3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // A0-AF
    3,  3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // B0-BF
    9,  9, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // C0-CF
    4,  4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // D0-DF
    10, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 5, 5, // E0-EF
    11, 7, 7, 7, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, // F0-FF
];

// This second table maps a state to a new state when adding a transition.
//  00-7F
//  |   80-8F
//  |   |   90-9F
//  |   |   |   A0-BF
//  |   |   |   |   C2-DF
//  |   |   |   |   |   E1-EC, EE, EF
//  |   |   |   |   |   |   ED
//  |   |   |   |   |   |   |   F1-F3
//  |   |   |   |   |   |   |   |   F4
//  |   |   |   |   |   |   |   |   |   C0, C1, F5-FF
//  |   |   |   |   |   |   |   |   |   |  E0
//  |   |   |   |   |   |   |   |   |   |  |   F0
static STATES: [u8; 108] = [
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 0,  0,   // REJECT = 0
    12, 0,  0,  0,  24, 36, 48, 60, 72, 0, 84, 96,  // ACCEPT = 12
    0,  12, 12, 12, 0,  0,  0,  0,  0,  0, 0,  0,   // 2-byte = 24
    0,  24, 24, 24, 0,  0,  0,  0,  0,  0, 0,  0,   // 3-byte = 36
    0,  24, 24, 0,  0,  0,  0,  0,  0,  0, 0,  0,   // 3-byte low/mid = 48
    0,  36, 36, 36, 0,  0,  0,  0,  0,  0, 0,  0,   // 4-byte = 60
    0,  36, 0,  0,  0,  0,  0,  0,  0,  0, 0,  0,   // 4-byte low = 72
    0,  0,  0,  24, 0,  0,  0,  0,  0,  0, 0,  0,   // 3-byte high = 84
    0,  0,  36, 36, 0,  0,  0,  0,  0,  0, 0,  0,   // 4-byte mid/high = 96

];

static STATE_ACCEPT: u8 = 12;
static STATE_REJECT: u8 = 0;

#[inline]
pub fn decode(byte: u8, state: &mut u8) -> bool {
    let transition = TRANSITIONS[byte as usize];
    let s = unsafe { *STATES.get_unchecked((*state + transition) as usize) };
    *state = s;
    s > STATE_ACCEPT
}

#[derive(Debug)]
pub struct Utf8Error {
    valid_up_to: usize,
    error_len: Option<u8>,
}

pub fn run_utf8_validation(v: &[u8], collect: bool) -> Result<Vec<usize>, Utf8Error> {
    let mut index = 0;
    let len = v.len();
    let mut indexes = vec![];

    while index < len {
        let old_offset = index;
        if collect {
            indexes.push(old_offset);
        }
        macro_rules! err {
            ($error_len: expr) => {
                return Err(Utf8Error {
                    valid_up_to: old_offset,
                    error_len: $error_len,
                });
            };
        }

        macro_rules! next {
            () => {{
                index += 1;
                // we needed data, but there was none: error!
                if index >= len {
                    err!(None)
                }
                v[index]
            }};
        }

        let first = v[index];
        if first < 0x80 {
            // I've shortened this to focus on the multi-byte sequences. This
            // should absolutey continue to use the NONASCII_MASK processing.
            index += 1;
        } else if first >= 0xC2 && first <= 0xF4 {
            // A first byte in the range of 0xC2..=0xF4 signals a multi-byte
            // sequence. Setup our DFA state to ingest the bytes.
            let mut state = STATE_ACCEPT;

            // Ingest the first byte and second bytes.
            // We know that the first byte is not an invalid byte due to the
            // range, so there's no need to check the state afterwards. We also
            // know we need at least 2 bytes.
            decode(first, &mut state);
            let needs_more = decode(next!(), &mut state);

            // If the decoding requires more bytes, ingest them.
            // If the second byte was all that was needed, we'll be back at
            // STATE_ACCEPT. Or, if the second byte was invalid, we'll be at
            // STATE_REJECT. Neither need more bytes.
            if needs_more {
                if decode(next!(), &mut state) {
                    // There are at most 4 bytes, so no need to check for
                    // needs_more.
                    decode(next!(), &mut state);
                }
            }

            // If we ingested an invalid sequence, then index will be the byte
            // that was invalid.
            if state == STATE_REJECT {
                err!(Some((index - old_offset) as u8));
            }
            index += 1;
        } else {
            err!(Some(0));
        }
    }

    Ok(indexes)
}
