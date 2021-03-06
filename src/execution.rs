//! Binary lambda calculus execution

use lambda_calculus::*;
use encoding::binary::from_bits;
use encoding::lambda::{encode, decode};
use self::Error::*;
use std::mem;
use std::collections::VecDeque;

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
    Bits(&'a [u8]),
    /// unencoded byte input
    Bytes(&'a [u8])
}

#[derive(Debug, Clone)]
struct Env(VecDeque<Closure>);

type Closure = (Term, Env);

type Stack = Env;

#[derive(Debug)]
struct State {
    term: Term,
    stack: Stack,
    env: Env
}

impl State {
    pub fn new(term: Term) -> Self {
        State {
            term:  term,
            stack: Env(VecDeque::new()),
            env:   Env(VecDeque::new())
        }
    }

    pub fn process(mut self) -> Self {
        let tmp = mem::replace(&mut self.term, Var(0));

        match tmp {
            App(lhs, rhs) => {
                self.stack.0.push_front((*rhs, self.env.clone()));
                mem::replace(&mut self.term, *lhs);
            },
            Abs(abs) => {
                mem::replace(&mut self.term, *abs);
                if let Some(t) = self.stack.0.pop_front() {
                    self.env.0.push_front(t)
                } else {
                    return self
                }
            },
            Var(1) => {
                if let Some((t, e)) = self.env.0.pop_front() {
                    self.term = t;
                    self.env = e;
                } else {
                    return self
                }
            },
            Var(n) => {
                if self.env.0.pop_front().is_some() {
                    mem::replace(&mut self.term, Var(n - 1));
                } else {
                    return self
                }
            }
        }

        self.process()
    }
}

/// Executes a binary lambda calculus program, optionally feeding it the given argument.
/// More programs can be found in the `tests` directory.
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
    let program = from_bits(blc_program);
    if program.is_err() { return Err(InvalidProgram) }
    let program = program.unwrap(); // safe

    let calculation = match input {
        Input::Nothing     => beta(program, NOR, 0),
        Input::Bytes(arg)  => beta(app(program, encode(arg)), NOR, 0),
        Input::Bits(arg) => {
            let arg = from_bits(arg);
            if arg.is_ok() {
                beta(app(program, arg.unwrap()), NOR, 0) // safe
            } else {
                return Err(InvalidArgument)
            }
        }
    };

    decode(calculation).or(Err(InvalidProgram))
}

/*
#[cfg(test)]
mod test {
    use super::*;
    use encoding::binary::{decompress, to_bits};

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
}
*/
