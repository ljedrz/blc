//! Binary encoding for lambda expressions

use lambda_calculus::term::*;
use lambda_calculus::term::Term::*;
use self::Error::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotATerm
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

/// Parse a binary-encoded lambda `Term`.
///
/// # Example
/// ```
/// use blc::binary_encoding::{from_binary, to_binary};
///
/// let k = from_binary(b"0000110");
///
/// assert!(k.is_ok());
/// assert_eq!(to_binary(&k.unwrap()), "0000110".to_owned());
/// ```
pub fn from_binary(input: &[u8]) -> Result<Term, Error> {
    if let Some((result, _)) = _from_binary(input) {
        Ok(result)
    } else {
        Err(NotATerm)
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
/// assert_eq!(to_binary(&k.unwrap()), "0000110".to_owned());
/// ```

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

pub fn to_binary(term: &Term) -> Vec<u8> {
    let mut output = Vec::new();
    _to_binary(term, &mut output);

    output
}

#[cfg(test)]
mod test {
    use super::*;

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

        let quine = Vec::from(&
            b"000101100100011010000000000001011011110010111100111111011111011010"[..]);
        let primes = Vec::from(&
            b"000100011001100101000110100000000101100000100100010101111101111010010001101\
              000011100110100000000001011011100111001111111011110000000011111001101110000\
              00101100000110110"[..]);
        let blc = Vec::from(&
            b"010100011010000000010101100000000001111000010111111001111000010111001111000\
              000111100001011011011100111110000111110000101111010011101001011001110000110\
              110000101111100001111100001110011011110111110011110111011000011001000110100\
              0011010"[..]);

        assert_eq!(to_binary(&from_binary(&*k).unwrap()),      k);
        assert_eq!(to_binary(&from_binary(&*v15).unwrap()),    v15);
        assert_eq!(to_binary(&from_binary(&*s).unwrap()),      s);
        assert_eq!(to_binary(&from_binary(&*succ).unwrap()),   succ);
        assert_eq!(to_binary(&from_binary(&*quine).unwrap()),  quine);
        assert_eq!(to_binary(&from_binary(&*primes).unwrap()), primes);
        assert_eq!(to_binary(&from_binary(&*blc).unwrap()),    blc);
    }
}
