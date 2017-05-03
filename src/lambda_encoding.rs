//! Lambda encoding for byte strings

use lambda_calculus::term::*;
use lambda_calculus::booleans::{tru, fls};
use std::char;

/// Decode lambda-encoded data as a `String`.
///
/// # Example
/// ```
/// use blc::binary_encoding::from_binary;
/// use blc::lambda_encoding::decode;
///
/// let k = from_binary(b"0000110");
///
/// assert!(k.is_ok());
/// assert_eq!(decode(k.unwrap()), "(λλ2)");
/// ```
pub fn decode(term: Term) -> String {
    if term == fls() {
        "".into()
    } else if term.is_list() && term.head_ref().unwrap().is_list() { // safe
        let (head, tail) = term.uncons().unwrap(); // safe
        let byte = decode_byte(head);
        let chr = char::from(byte);
        chr.to_string() + &decode(tail)
    } else if term.head_ref() == Ok(&fls()) {
        "1".to_string() + &decode(term.tail().unwrap()) // safe
    } else if term.head_ref() == Ok(&tru()) {
        "0".to_string() + &decode(term.tail().unwrap()) // safe
    } else {
        format!("({})", term)
    }
}

// TODO: make safer
fn decode_byte(encoded_byte: Term) -> u8 {
    let bits = encoded_byte
        .into_iter()
        .map(|t| (t
            .unabs()
            .and_then(|t| t.unabs())
            .and_then(|t| t.unvar())
            .expect("not a lambda-encoded byte!") - 1) as u8
        );

    !bits.fold(0, |acc, b| acc * 2 + b)
}

fn encode_byte(byte: u8) -> Term {
    let bitstr = format!("{:08b}", byte);
    let bits = bitstr.as_bytes();
    Term::from(bits.into_iter().map(|&bit| encode_bit(bit)).collect::<Vec<Term>>())
}

fn encode_bit(bit: u8) -> Term {
    match bit {
        b'0' => tru(),
        b'1' => fls(),
        _ => unreachable!()
    }
}

/// Encode bytes as a lambda `Term`.
///
/// # Example
/// ```
/// use blc::lambda_encoding::encode;
///
/// assert_eq!(&*format!("{}", encode(b"a")),
///     "λ1(λ1(λλ2)(λ1(λλ1)(λ1(λλ1)(λ1(λλ2)(λ1(λλ2)(λ1(λλ2)(λ1(λλ2)(λ1(λλ1)(λλ1)))))))))(λλ1)");
/// ```
pub fn encode(input: &[u8]) -> Term {
    Term::from(input.into_iter().map(|&b| encode_byte(b)).collect::<Vec<Term>>())
}

#[cfg(test)]
mod test {
    use super::*;
    use binary_encoding::from_binary;
    use std::str;

    #[test]
    fn encoding_lambda() {
        assert_eq!(&*format!("{}", encode(b"0")),
            "λ1(λ1(λλ2)(λ1(λλ2)(λ1(λλ1)(λ1(λλ1)(λ1(λλ2)(λ1(λλ2)(λ1(λλ2)(λ1(λλ2)(λλ1)))))))))(λλ1)");
        assert_eq!(&*format!("{}", encode(b"1")),
            "λ1(λ1(λλ2)(λ1(λλ2)(λ1(λλ1)(λ1(λλ1)(λ1(λλ2)(λ1(λλ2)(λ1(λλ2)(λ1(λλ1)(λλ1)))))))))(λλ1)");
        assert_eq!(&*format!("{}", encode(b"a")),
            "λ1(λ1(λλ2)(λ1(λλ1)(λ1(λλ1)(λ1(λλ2)(λ1(λλ2)(λ1(λλ2)(λ1(λλ2)(λ1(λλ1)(λλ1)))))))))(λλ1)");
    }

    #[test]
    fn decoding_lambda() {
        let k =     from_binary(b"0000110").unwrap();
        let s =     from_binary(b"00000001011110100111010").unwrap();
        let quine = from_binary(b"000101100100011010000000000001011\
                                  011110010111100111111011111011010").unwrap();

        assert_eq!(decode(k),     "(λλ2)");
        assert_eq!(decode(s),     "(λλλ31(21))");
        assert_eq!(decode(quine), "(λ1((λ11)(λλλλλ14(3(55)2)))1)");
    }

    #[test]
    fn decode_encode_lambda() {
        assert_eq!(decode(encode(b"herp derp")),             "herp derp");
        assert_eq!(decode(encode(b"0111010101011")),         "0111010101011");
        assert_eq!(decode(encode(b"01zeros110and1ones101")), "01zeros110and1ones101");
        assert_eq!(decode(encode(b"\0(1)")),                 "\0(1)");
    }
}
