//! Binary encoding for lambda expressions

use lambda_calculus::term::*;
use lambda_calculus::term::Term::*;
use self::Error::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotATerm
}

/// Parse a binary-encoded lambda `Term`.
///
/// # Example
/// ```
/// use blc::binary_encoding::{from_binary, to_binary};
///
/// let k = from_binary(b"0000110");
///
/// assert!(k.is_ok());
/// assert_eq!(to_binary(&k.unwrap()), Vec::from(&b"0000110"[..]));
/// ```
pub fn from_binary(input: &[u8]) -> Result<Term, Error> {
    if let Some((result, _)) = _from_binary(input) {
        Ok(result)
    } else {
        Err(NotATerm)
    }
}

fn _from_binary(input: &[u8]) -> Option<(Term, &[u8])> {
    if input.len() == 0 { return None }

    if [9, 10, 13, 32].contains(&input[0]) {
        _from_binary(&input[1..])
    } else {
        match &input[0..2] {
            b"00" => {
                if let Some((term, rest)) = _from_binary(&input[2..]) {
                    Some((abs(term), rest))
                } else {
                    None
                }
            },
            b"01" => {
                if let Some((term1, rest1)) = _from_binary(&input[2..]) {
                    if let Some((term2, rest2)) = _from_binary(&rest1) {
                        Some((app(term1, term2), &rest2))
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            b"10" | b"11" => {
                let i = input.iter().take_while(|&b| *b == b'1').count();
                if input[2..].len() == 0 {
                    Some((Var(i), &*b""))
                } else {
                    Some((Var(i), &input[i+1..]))
                }
            },
            _ => None
        }
    }
}

/// Represent a lambda `Term` in binary.
///
/// # Example
/// ```
/// use blc::binary_encoding::{from_binary, to_binary};
///
/// let k = from_binary(b"0000110");
///
/// assert!(k.is_ok());
/// assert_eq!(to_binary(&k.unwrap()), Vec::from(&b"0000110"[..]));
/// ```
pub fn to_binary(term: &Term) -> Vec<u8> {
    let mut output = Vec::new();
    _to_binary(term, &mut output);

    output
}

fn _to_binary(term: &Term, output: &mut Vec<u8>) {
    match *term {
        Var(i) => {
            for _ in 0..i { output.push(b'1') }
            output.push(b'0');
        }
        Abs(ref t) => {
            output.extend_from_slice(b"00");
            output.append(&mut to_binary(t));
        }
        App(ref t1, ref t2) => {
            output.extend_from_slice(b"01");
            output.append(&mut to_binary(t1));
            output.append(&mut to_binary(t2));
        }
    }
}

/// Convert a stream of "bits" into bytes. It is not ideally reversible with `decompress`, because
/// it always produces full bytes, while the length of its input can be indivisible by 8.
///
/// # Example
/// ```
/// use blc::binary_encoding::{compress};
///
/// let succ_compressed = compress(&*b"000000011100101111011010");
/// assert_eq!(succ_compressed, vec![0x1, 0xCB, 0xDA]);
/// ```
pub fn compress(binary: &[u8]) -> Vec<u8> {
    let length = binary.len();
    let mut output = Vec::with_capacity(length / 8 + 1);
    let mut pos = 0;

    while pos <= length - 8 {
        output.push(bits_to_byte(&binary[pos..(pos + 8)]));
        pos += 8;
    }

    if pos != length {
        let mut last_byte = Vec::with_capacity(8);
        last_byte.extend_from_slice(&binary[pos..]);
        for _ in 0..(8 - (length - pos)) { last_byte.push(b'0') }
        output.push(bits_to_byte(&last_byte));
    }

    output
}

fn bits_to_byte(bits: &[u8]) -> u8 {
    bits.iter().fold(0, |acc, &b| acc * 2 + (b - 48))
}

/// Convert bytes into "bits" suitable for binary lambda calculus purposes.
///
/// # Example
/// ```
/// use blc::binary_encoding::{decompress};
///
/// let succ_compressed = vec![0x1, 0xCB, 0xDA];
/// assert_eq!(decompress(&succ_compressed), b"000000011100101111011010");
/// ```
pub fn decompress(bytes: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(bytes.len() * 8);

    for byte in bytes {
        println!("byte {:x} = {:08b}", byte, byte);
        output.extend_from_slice(format!("{:08b}", byte.to_be()).as_bytes());
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;

    const QUINE: &'static [u8; 66] =
        b"000101100100011010000000000001011011110010111100111111011111011010";

    const PRIMES: &'static [u8; 167] =
        b"00010001100110010100011010000000010110000010010001010111110111101001000110100001\
          11001101000000000010110111001110011111110111100000000111110011011100000010110000\
          0110110";

    const BLC: &'static [u8; 232] =
        b"01010001101000000001010110000000000111100001011111100111100001011100111100000011\
          11000010110110111001111100001111100001011110100111010010110011100001101100001011\
          111000011111000011100110111101111100111101110110000110010001101000011010";

    #[test]
    fn variables() {
        assert_eq!(from_binary(b"10"),   Ok(Var(1)));
        assert_eq!(from_binary(b"110"),  Ok(Var(2)));
        assert_eq!(from_binary(b"1110"), Ok(Var(3)));
    }

    #[test]
    fn abstractions() {
        assert_eq!(from_binary(b"0010"),     Ok(abs(Var(1))));
        assert_eq!(from_binary(b"000010"),   Ok(abs(abs(Var(1)))));
        assert_eq!(from_binary(b"00000010"), Ok(abs(abs(abs((Var(1)))))));
    }

    #[test]
    fn applications() {
        assert_eq!(from_binary(b"011010"),  Ok(app(Var(1), Var(1))));
        assert_eq!(from_binary(b"0110110"), Ok(app(Var(1), Var(2))));
        assert_eq!(from_binary(b"0111010"), Ok(app(Var(2), Var(1))));
    }

    #[test]
    fn ignoring_whitespaces() {
        assert_eq!(from_binary(b"00 00\t00\n10\r\n"), Ok(abs(abs(abs((Var(1)))))));
    }

    #[test]
    fn from_binary_and_back() {
        let k =    b"0000110";
        let v15 =  b"1111111111111110";
        let s =    b"00000001011110100111010";
        let succ = b"000000011100101111011010";

        assert_eq!(to_binary(&from_binary(&*k).unwrap()),      k);
        assert_eq!(to_binary(&from_binary(&*v15).unwrap()),    v15);
        assert_eq!(to_binary(&from_binary(&*s).unwrap()),      s);
        assert_eq!(to_binary(&from_binary(&*succ).unwrap()),   succ);
        assert_eq!(to_binary(&from_binary(&*QUINE).unwrap()),  &QUINE[..]);
        assert_eq!(to_binary(&from_binary(&*PRIMES).unwrap()), Vec::from(&PRIMES[..]));
        assert_eq!(to_binary(&from_binary(&*BLC).unwrap()),    Vec::from(&BLC[..]));
    }

    #[test]
    fn compression() {
        let primes_c = compress(&PRIMES[..]);
        assert_eq!(primes_c.first().unwrap(), &0x11);
        assert_eq!(primes_c.last().unwrap(),  &0x6c);

        let blc_c = compress(&BLC[..]);
        assert_eq!(blc_c.first().unwrap(), &0x51);
        assert_eq!(blc_c.last().unwrap(),  &0x1a);
    }

    #[test]
    fn decompression() {
        let s_c = vec![0x1, 0x7a, 0x74];
        assert_eq!(compress(&decompress(&s_c)), s_c);
    }

    #[test]
    fn compress_decompress() {
        assert_eq!(decompress(&compress(&BLC[..])), Vec::from(&BLC[..]));
    }

}
