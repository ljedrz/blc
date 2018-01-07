//! Binary lambda calculus execution

use lambda_calculus::*;
use binary_encoding::from_binary;
use lambda_encoding::{encode, decode};
use self::Error::*;
use self::Input::*;

/// An error that can occur during BLC execution.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// invalid BLC program
    InvalidProgram,
    /// invalid BLC argument
    InvalidArgument
}

/// The type of input for BLC execution.
pub enum Input<'a> {
    /// no input parameter
    Nothing,
    /// BLC input
    Binary(&'a [u8]),
    /// unencoded byte input
    Bytes(&'a [u8])
}

/// Executes a binary lambda calculus program, optionally feeding it the given argument.
///
/// # Example
/// ```
/// use blc::execution::run;
/// use blc::execution::Input::Bytes;
///
/// let reverse_blc = b"0001011001000110100000000001011100111110111100001011011110110000010";
///
/// assert_eq!(run(&*reverse_blc, Bytes(b"herp derp")), Ok("pred preh".into()));
/// ```
pub fn run(blc_program: &[u8], input: Input) -> Result<String, Error> {
    let program = from_binary(blc_program);
    if program.is_err() { return Err(InvalidProgram) }
    let program = program.unwrap(); // safe

    let calculation = match input {
        Nothing     => beta(program, NOR, 0),
        Bytes(arg)  => beta(app(program, encode(arg)), NOR, 0),
        Binary(arg) => {
            let arg = from_binary(arg);
            if arg.is_ok() {
                beta(app(program, arg.unwrap()), NOR, 0) // safe
            } else {
                return Err(InvalidArgument)
            }
        }
    };

    Ok(decode(calculation))
}

#[cfg(test)]
mod test {
    use super::*;
    use binary_encoding::{decompress, to_binary};
    use lambda_calculus::data::num::church::{is_zero, rem};

    #[test]
    fn id() {
        let id_compressed = b" ";
        let id_blc        = decompress(&*id_compressed);
        let input         = b"herp derp";

        assert_eq!(run(&*id_blc, Bytes(&*input)).unwrap(), "herp derp".to_owned());
    }

    #[test]
    fn sort() {
        let sort_compressed =
            [0x15, 0x46, 0x84, 0x06, 0x05, 0x46, 0x81, 0x60, 0x15, 0xfb, 0xec, 0x2f, 0x80, 0x01,
             0x5b, 0xf9, 0x7f, 0x0b, 0x7e, 0xf7, 0x2f, 0xec, 0x2d, 0xfb, 0x80, 0x56, 0x05, 0xfd,
             0x85, 0xbb, 0x76, 0x11, 0x5d, 0x50, 0x5c, 0x00, 0xbe, 0x7f, 0xc1, 0x2b, 0xff, 0x0f,
             0xfc, 0x2c, 0x1b, 0x72, 0xbf, 0xf0, 0xff, 0xc2, 0xc1, 0x6d, 0x34, 0x50, 0x40];

        let sort_blc = decompress(&sort_compressed);
        let input  = b"3241";

        assert_eq!(run(&*sort_blc, Bytes(&*input)).unwrap(), "1234".to_owned());
    }

    #[test]
    fn quine() {
        // program code from https://tromp.github.io/cl/Binary_lambda_calculus.html#A_quine
        let quine_blc = b"000101100100011010000000000001011\
            011110010111100111111011111011010";
        let input = b"hurr";

        assert_eq!(run(&quine_blc[..], Bytes(&input[..])), Ok("hurrhurr".to_owned()));
    }

    #[test]
    fn inflating() {
        // program code from http://www.ioccc.org/2012/tromp/inflate.Blc
        let inflate_compressed =
            [0x44, 0x44, 0x68, 0x16, 0x01, 0x79, 0x1a, 0x00, 0x16, 0x7f, 0xfb, 0xcb, 0xcf, 0xdf,
             0x65, 0xfb, 0xed, 0x0f, 0x3c, 0xe7, 0x3c, 0xf3, 0xc2, 0xd8, 0x20, 0x58, 0x2c, 0x0b,
             0x06, 0xc0];

        let inflate_blc = decompress(&inflate_compressed);
        let s_compressed = [0x1, 0x7a, 0x74];

        assert_eq!(run(&*inflate_blc, Bytes(&s_compressed[..])).unwrap(), "000000010111101001110100".to_owned());
    }

    #[test]
    fn deflating() {
        // program code from http://www.ioccc.org/2012/tromp/deflate.Blc
        let deflate_compressed =
            [0x44, 0x68, 0x16, 0x05, 0x7e, 0x01, 0x17, 0x00, 0xbe, 0x55, 0xff, 0xf0, 0x0d, 0xc1,
             0x8b, 0xb2, 0xc1, 0xb0, 0xf8, 0x7c, 0x2d, 0xd8, 0x05, 0x9e, 0x09, 0x7f, 0xbf, 0xb1,
             0x48, 0x39, 0xce, 0x81, 0xce, 0x80];

        let deflate_blc = decompress(&deflate_compressed);
        let s_blc = b"00000001011110100111010";

        assert_eq!(run(&*deflate_blc, Bytes(&s_blc[..])).unwrap().as_bytes(), [0x1, 0x7a, 0x74]);
    }

    #[test]
    fn fizz_buzz() {
        let fizzbuzz_single =
            abs(
                app!(
                    is_zero(),
                    app!(rem(), Var(1), 15.into_church()),
                    encode(&b"FizzBuzz"[..]),
                    app!(
                        is_zero(),
                        app!(rem(), Var(1), 3.into_church()),
                        encode(&b"Fizz"[..]),
                        app!(
                            is_zero(),
                            app!(rem(), Var(1), 5.into_church()),
                            encode(&b"Buzz"[..]),
                            Var(1)
                        )
                    )
                )
            );
        let fizzbuzz_blc = to_binary(&fizzbuzz_single);

        assert_eq!(run(&*fizzbuzz_blc, Binary(&to_binary(&1.into_church()))).unwrap(),  "(λλ21)");
        assert_eq!(run(&*fizzbuzz_blc, Binary(&to_binary(&2.into_church()))).unwrap(),  "(λλ2(21))");
        assert_eq!(run(&*fizzbuzz_blc, Binary(&to_binary(&3.into_church()))).unwrap(),  "Fizz");
        assert_eq!(run(&*fizzbuzz_blc, Binary(&to_binary(&4.into_church()))).unwrap(),  "(λλ2(2(2(21))))");
        assert_eq!(run(&*fizzbuzz_blc, Binary(&to_binary(&5.into_church()))).unwrap(),  "Buzz");
        assert_eq!(run(&*fizzbuzz_blc, Binary(&to_binary(&15.into_church()))).unwrap(), "FizzBuzz");
    }

/*
    #[test] /* WIP; this one parses properly, but doesn't return the expected result */
    fn hilbert() {
        // program code from http://www.ioccc.org/2012/tromp/hilbert.Blc
        let hilbert_compressed =
            [0x18, 0x18, 0x18, 0x18, 0x11, 0x11, 0x54, 0x68, 0x06, 0x04, 0x15, 0x5f, 0xf0, 0x41,
             0x9d, 0xf9, 0xde, 0x16, 0xff, 0xfe, 0x5f, 0x3f, 0xef, 0xf6, 0x15, 0xff, 0x94, 0x68,
             0x40, 0x58, 0x11, 0x7e, 0x05, 0xcb, 0xfe, 0xbc, 0xbf, 0xee, 0x86, 0xcb, 0x94, 0x68,
             0x16, 0x00, 0x5c, 0x0b, 0xfa, 0xcb, 0xfb, 0xf7, 0x1a, 0x85, 0xe0, 0x5c, 0xf4, 0x14,
             0xd5, 0xfe, 0x08, 0x18, 0x0b, 0x04, 0x8d, 0x08, 0x00, 0xe0, 0x78, 0x01, 0x64, 0x45,
             0xff, 0xe5, 0xff, 0x7f, 0xff, 0xfe, 0x5f, 0xff, 0x2f, 0xc0, 0x2f, 0x7a, 0xd9, 0x7f,
             0x5b, 0xff, 0xff, 0xfb, 0xff, 0xfc, 0xaa, 0xff, 0xf7, 0x81, 0x7f, 0xfa, 0xdf, 0x76,
             0x69, 0x54, 0x68, 0x06, 0x01, 0x57, 0xf7, 0xe1, 0x60, 0x5c, 0x13, 0xfe, 0x80, 0xb2,
             0x2c, 0x18, 0x58, 0x1b, 0xfe, 0x5c, 0x10, 0x42, 0xff, 0x80, 0x5d, 0xee, 0xc0, 0x6c,
             0x2c, 0x0c, 0x06, 0x08, 0x19, 0x1a, 0x00, 0x16, 0x7f, 0xbc, 0xbc, 0xfd, 0xf6, 0x5f,
             0x7c, 0x0a, 0x20];

        let hilbert_blc = decompress(&hilbert_compressed);
        let arg = b"1234";

        assert_eq!(run(&hilbert_blc[..], Bytes(&arg[..])), Ok("WIP".into()));
    }
*/
/*
    #[test] /* WIP; this just returns λ1 */
    fn brainfuck() {
        // program code from http://www.ioccc.org/2012/tromp/bf.Blc
        let bf_interpreter_compressed =
            [0x44, 0x51, 0xa1, 0x01, 0x84, 0x55, 0xd5, 0x02, 0xb7, 0x70, 0x30, 0x22, 0xff, 0x32,
             0xf0, 0x00, 0xbf, 0xf9, 0x85, 0x7f, 0x5e, 0xe1, 0x6f, 0x95, 0x7f, 0x7d, 0xee, 0xc0,
             0xe5, 0x54, 0x68, 0x00, 0x58, 0x55, 0xfd, 0xfb, 0xe0, 0x45, 0x57, 0xfd, 0xeb, 0xfb,
             0xf0, 0xb6, 0xf0, 0x2f, 0xd6, 0x07, 0xe1, 0x6f, 0x73, 0xd7, 0xf1, 0x14, 0xbc, 0xc0,
             0x0b, 0xff, 0x2e, 0x1f, 0xa1, 0x6f, 0x66, 0x17, 0xe8, 0x5b, 0xef, 0x2f, 0xcf, 0xff,
             0x13, 0xff, 0xe1, 0xca, 0x34, 0x20, 0x0a, 0xc8, 0xd0, 0x0b, 0x99, 0xee, 0x1f, 0xe5,
             0xff, 0x7f, 0x5a, 0x6a, 0x1f, 0xff, 0x0f, 0xff, 0x87, 0x9d, 0x04, 0xd0, 0xab, 0x00,
             0x05, 0xdb, 0x23, 0x40, 0xb7, 0x3b, 0x28, 0xcc, 0xc0, 0xb0, 0x6c, 0x0e, 0x74, 0x10];

        let bf_interpreter_blc = decompress(&bf_interpreter_compressed);
        let bf_hello =
            b"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

        assert_eq!(run(&bf_interpreter_blc, Bytes(&bf_hello[..])), Ok("Hello World!".into()));
    }
*/
}
