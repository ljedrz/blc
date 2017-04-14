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
pub fn to_binary(term: &Term) -> String {
    match *term {
        Var(i) => format!("{}0", "1".repeat(i)),
        Abs(ref t) => format!("00{}", to_binary(t)),
        App(ref t1, ref t2) => format!("01{}{}", to_binary(t1), to_binary(t2))
    }
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
    fn parse_and_display() {
        let k =      b"0000110";
        let v15 =    b"1111111111111110";
        let s =      b"00000001011110100111010";
        let succ =   b"000000011100101111011010";
        let quine =  b"000101100100011010000000000001011011110010111100111111011111011010";
        let primes = b"000100011001100101000110100000000101100000100100010101111101111010010001101\
                       000011100110100000000001011011100111001111111011110000000011111001101110000\
                       00101100000110110";
        let blc =    b"010100011010000000010101100000000001111000010111111001111000010111001111000\
                       000111100001011011011100111110000111110000101111010011101001011001110000110\
                       110000101111100001111100001110011011110111110011110111011000011001000110100\
                       0011010";

        assert_eq!(format!("{}", from_binary(&*k).unwrap()),      "λλ2");
        assert_eq!(format!("{}", from_binary(&*v15).unwrap()),    "F");
        assert_eq!(format!("{}", from_binary(&*s).unwrap()),      "λλλ31(21)");
        assert_eq!(format!("{}", from_binary(&*succ).unwrap()),   "λλλ2(321)");
        assert_eq!(format!("{}", from_binary(&*quine).unwrap()),  "λ1((λ11)(λλλλλ14(3(55)2)))1");
        assert_eq!(format!("{}", from_binary(&*primes).unwrap()), "λ(λ1(1((λ11)(λλλ1(λλ1)((λ441((λ\
            11)(λ2(11))))(λλλλ13(2(64)))))(λλλ4(13)))))(λλ1(λλ2)2)");
        assert_eq!(format!("{}", from_binary(&*blc).unwrap()),    "(λ11)(λλλ1(λλλλ3(λ5(3(λ2(3(λλ3(\
            λ123)))(4(λ4(λ31(21))))))(1(2(λ12))(λ4(λ4(λ2(14)))5))))(33)2)(λ1((λ11)(λ11)))");
    }
}
