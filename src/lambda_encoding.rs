//! Lambda encoding for byte strings

use lambda_calculus::term::*;
use lambda_calculus::booleans::{tru, fls};
use lambda_calculus::list::nil;
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
    } else if term.is_list() && term.head_ref().unwrap().is_list() {
        let (head, tail) = term.uncons().unwrap();
        let terms: Vec<Term> = head.into_iter().collect();
        let bits = terms
                   .into_iter()
                   .map(|t| (t
                            .unabs()
                            .and_then(|t| t.unabs())
                            .and_then(|t| t.unvar())
                            .unwrap() - 1) as u8
                    ).collect::<Vec<u8>>();
        let byte = !bits.iter().fold(0, |acc, &b| acc * 2 + b);
        let chr = char::from(byte);
        chr.to_string() + &decode(tail)
    } else if term.head_ref() == Ok(&fls()) {
        "1".to_string() + &decode(term.tail().unwrap())
    } else if term.head_ref() == Ok(&tru()) {
        "0".to_string() + &decode(term.tail().unwrap())
    } else {
        format!("({})", term)
    }
}

fn encode_byte(b: u8) -> Term {
    match b {
        b'0' => tru(),
        b'1' => fls(),
        x    => encode(&format!("{:08b}", x).as_bytes())
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
    input.iter().rev().fold(nil(), |acc, &b| acc.push(encode_byte(b)))
}

#[cfg(test)]
mod test {
    use super::*;
    use binary_encoding::{from_binary, to_binary};
    use std::str;

    #[test]
    fn encoding_lambda() {
        assert_eq!(&*format!("{}", encode(b"0")),     "λ1(λλ2)(λλ1)");
        assert_eq!(&*format!("{}", encode(b"1")),     "λ1(λλ1)(λλ1)");
        assert_eq!(&*format!("{}", encode(b"00")),    "λ1(λλ2)(λ1(λλ2)(λλ1))");
        assert_eq!(&*format!("{}", encode(b"01")),    "λ1(λλ2)(λ1(λλ1)(λλ1))");
        assert_eq!(&*format!("{}", encode(b"10")),    "λ1(λλ1)(λ1(λλ2)(λλ1))");
        assert_eq!(&*format!("{}", encode(b"11")),    "λ1(λλ1)(λ1(λλ1)(λλ1))");
        assert_eq!(&*format!("{}", encode(b"001")),   "λ1(λλ2)(λ1(λλ2)(λ1(λλ1)(λλ1)))");
        assert_eq!(&*format!("{}", encode(b"100")),   "λ1(λλ1)(λ1(λλ2)(λ1(λλ2)(λλ1)))");
        assert_eq!(&*format!("{}", encode(b"a")),     "λ1(λ1(λλ2)(λ1(λλ1)(λ1(λλ1)(λ1(λλ2)(λ1(λλ2)\
                                                       (λ1(λλ2)(λ1(λλ2)(λ1(λλ1)(λλ1)))))))))(λλ1)");
        assert_eq!(&*format!("{}", encode(b"z0")),    "λ1(λ1(λλ2)(λ1(λλ1)(λ1(λλ1)(λ1(λλ1)(λ1(λλ1)\
                                                       (λ1(λλ2)(λ1(λλ1)(λ1(λλ2)(λλ1)))))))))(λ1(λλ\
                                                       2)(λλ1))");
        assert_eq!(&*format!("{}", encode(b"\0(1)")), "λ1(λ1(λλ2)(λ1(λλ2)(λ1(λλ2)(λ1(λλ2)(λ1(λλ2)(\
                                                       λ1(λλ2)(λ1(λλ2)(λ1(λλ2)(λλ1)))))))))(λ1(λ1(\
                                                       λλ2)(λ1(λλ2)(λ1(λλ1)(λ1(λλ2)(λ1(λλ1)(λ1(λλ2\
                                                       )(λ1(λλ2)(λ1(λλ2)(λλ1)))))))))(λ1(λλ1)(λ1(λ\
                                                       1(λλ2)(λ1(λλ2)(λ1(λλ1)(λ1(λλ2)(λ1(λλ1)(λ1(λ\
                                                       λ2)(λ1(λλ2)(λ1(λλ1)(λλ1)))))))))(λλ1))))");
    }

    #[test]
    fn decoding() {
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

    #[test]
    fn encoding_binary() {
        let s = from_binary(b"00000001011110100111010").unwrap();

        assert_eq!(to_binary(&s), "00000001011110100111010".to_owned())
    }
}
