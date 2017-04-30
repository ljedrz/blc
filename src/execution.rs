use binary_encoding::{from_binary};
use lambda_encoding::{encode, decode};
use lambda_calculus::reduction::beta_full;
use self::Error::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidProgram
}

pub fn execute(blc_program: &[u8], blc_argument: &[u8]) -> Result<String, Error> {
    let program = from_binary(blc_program);
    if program.is_err() { return Err(InvalidProgram) } 
    
    let calculation = beta_full(program.unwrap().app(encode(blc_argument))); // safe
    
    Ok(decode(calculation))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tmp() {
        let inflate = b"010001000100010001101000000101100000000101111001000110100000000000010110011111111111101111001011110011111101111101100101111110111110110100001111001111001110011100111100111100111100001011011000001000000101100000101100000010110000011011000000";
        let argument = [0x1u8, 0x7a, 0x74];
        
        assert_eq!(execute(&*inflate, &argument[..]), Ok("000000010111101001110100".into()));
    }
}
