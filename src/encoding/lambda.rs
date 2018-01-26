//! Lambda encoding for strings of bytes

use lambda_calculus::term::*;
use lambda_calculus::data::boolean::{tru, fls};
use encoding::binary::Error;
use std::char;
use pair_list::*;

/// Decode lambda-encoded data as a `String`.
///
/// # Example
/// ```
/// use blc::encoding::binary::from_bits;
/// use blc::encoding::lambda::decode;
///
/// let k = from_bits(b"0000110").unwrap();
///
/// assert_eq!(decode(k).unwrap(), "(λλ2)");
/// ```
pub fn decode(term: Term) -> Result<String, Error> {
    if term == fls() {
        Ok("".into())
    } else if is_list(&term) && is_list(head_ref(&term).unwrap()) { // safe
        let (head, tail) = uncons(term).unwrap(); // safe
        let byte = decode_byte(head)?;
        let chr = char::from(byte);
        Ok(chr.to_string() + &decode(tail)?)
    } else if head_ref(&term) == Ok(&fls()) {
        Ok("1".to_string() + &decode(tail(term).unwrap())?) // safe
    } else if head_ref(&term) == Ok(&tru()) {
        Ok("0".to_string() + &decode(tail(term).unwrap())?) // safe
    } else {
        Ok(format!("({:?})", term))
    }
}

fn decode_byte(encoded_byte: Term) -> Result<u8, Error> {
    let bits = vectorize_list(encoded_byte)
        .into_iter()
        .map(|t| t.unabs().and_then(|t| t.unabs()).and_then(|t| t.unvar()))
        .collect::<Vec<Result<usize, TermError>>>();

    if bits.iter().any(|b| b.is_err()) { return Err(Error::NotATerm) }

    Ok(!bits.into_iter().map(|b| (b.unwrap() - 1) as u8).fold(0, |acc, b| acc * 2 + b))
}

fn encode_byte(byte: u8) -> Term {
    let bitstr = format!("{:08b}", byte);
    let bits = bitstr.as_bytes();
    listify_terms(bits.into_iter().map(|&bit| encode_bit(bit)).collect::<Vec<Term>>())
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
/// use blc::encoding::lambda::encode;
///
/// assert_eq!(
///     &*format!("{:?}", encode(b"a")),
///     "λ1(λ1(λλ2)(λ1(λλ1)(λ1(λλ1)(λ1(λλ2)(λ1(λλ2)(λ1(λλ2)(λ1(λλ2)(λ1(λλ1)(λλ1)))))))))(λλ1)"
/// );
/// ```
pub fn encode(input: &[u8]) -> Term {
    listify_terms(input.into_iter().map(|&b| encode_byte(b)).collect::<Vec<Term>>())
}

#[cfg(test)]
mod test {
    use super::*;
    use encoding::binary::from_bits;

    #[test]
    fn encoding_lambda() {
        assert_eq!(
            &*format!("{:?}", encode(b"0")),
            "λ1(λ1(λλ2)(λ1(λλ2)(λ1(λλ1)(λ1(λλ1)(λ1(λλ2)(λ1(λλ2)(λ1(λλ2)(λ1(λλ2)(λλ1)))))))))(λλ1)"
        );

        assert_eq!(
            &*format!("{:?}", encode(b"1")),
            "λ1(λ1(λλ2)(λ1(λλ2)(λ1(λλ1)(λ1(λλ1)(λ1(λλ2)(λ1(λλ2)(λ1(λλ2)(λ1(λλ1)(λλ1)))))))))(λλ1)"
        );

        assert_eq!(
            &*format!("{:?}", encode(b"a")),
            "λ1(λ1(λλ2)(λ1(λλ1)(λ1(λλ1)(λ1(λλ2)(λ1(λλ2)(λ1(λλ2)(λ1(λλ2)(λ1(λλ1)(λλ1)))))))))(λλ1)"
        );
    }

    #[test]
    fn decoding_lambda() {
        let k =     from_bits(b"0000110").unwrap();
        let s =     from_bits(b"00000001011110100111010").unwrap();
        let quine = from_bits(b"000101100100011010000000000001011\
                                  011110010111100111111011111011010").unwrap();

        assert_eq!(decode(k).unwrap(),     "(λλ2)");
        assert_eq!(decode(s).unwrap(),     "(λλλ31(21))");
        assert_eq!(decode(quine).unwrap(), "(λ1((λ11)(λλλλλ14(3(55)2)))1)");
    }

    #[test]
    fn decode_encode_lambda() {
        assert_eq!(decode(encode(b"herp derp")).unwrap(),             "herp derp");
        assert_eq!(decode(encode(b"0111010101011")).unwrap(),         "0111010101011");
        assert_eq!(decode(encode(b"01zeros110and1ones101")).unwrap(), "01zeros110and1ones101");
        assert_eq!(decode(encode(b"\0(1)")).unwrap(),                 "\0(1)");
    }
}
