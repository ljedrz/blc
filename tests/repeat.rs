extern crate blc;

use blc::*;
use blc::encoding::binary::decompress;
use blc::execution::Input;

#[test]
// program code from https://tromp.github.io/cl/Binary_lambda_calculus.html#A_quine
fn repeat() {
    let code_compressed = [0x16, 0x46, 0x80, 0x05, 0xbc, 0xbc, 0xfd, 0xf6, 0x80];
    let code_blc        = decompress(&code_compressed);

    assert_eq!(
        run(&*code_blc, Input::Bytes(&*b"hurr")).unwrap(),
        "hurrhurr"
    );
}
