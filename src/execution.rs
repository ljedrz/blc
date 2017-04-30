use binary_encoding::{from_binary};
use lambda_encoding::{encode, decode};
use lambda_calculus::reduction::beta_full;
use self::Error::*;

/// An error that can occur during blc execution.
#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidProgram
}

/// Executes a binary lambda calculus program, feeding it the given argument.
///
/// # Example
/// ```
/// use blc::execution::run;
///
/// let reverse = b"0001011001000110100000000001011100111110111100001011011110110000010";
///
/// assert_eq!(run(&*reverse, b"herp derp"), Ok("pred preh".into()));
/// ```
pub fn run(blc_program: &[u8], blc_argument: &[u8]) -> Result<String, Error> {
    let program = from_binary(blc_program);
    if program.is_err() { return Err(InvalidProgram) }
    let calculation = beta_full(program.unwrap().app(encode(blc_argument))); // safe
    
    Ok(decode(calculation))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn inflating() {
        // program code from http://www.ioccc.org/2012/tromp/inflate.Blc
        let inflate = b"01000100010001000110100000010110000000010111100100011010000000000001011001\
            11111111111011110010111100111111011111011001011111101111101101000011110011110011100111\
            00111100111100111100001011011000001000000101100000101100000010110000011011000000";
        let s_compressed = [0x1u8, 0x7a, 0x74];
        
        assert_eq!(run(&*inflate, &s_compressed[..]).unwrap(), "000000010111101001110100".to_owned());
    }
  /*  
    #[test]
    fn deflating() {
        // program code from http://www.ioccc.org/2012/tromp/deflate.Blc
        let deflate = b"01000100011010000001011000000101011111100000000100010111000000001011111001\
            01010111111111111100000000110111000001100010111011001011000001101100001111100001111100\
            00101101110110000000010110011110000010010111111110111111101100010100100000111001110011\
            10100000011100111010000000";
            
        let s_blc = b"00000001011110100111010";
        
        assert_eq!(run(&*deflate, &s_blc[..]).unwrap().as_bytes(), [0x1u8, 0x7a, 0x74]);
    }*/
}
