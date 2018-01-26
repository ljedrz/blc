extern crate blc;

use blc::*;
use blc::encoding::binary::decompress;
use blc::execution::Input;

#[test]
fn identity() {
    let code_compressed = b" ";
    let code_blc        = decompress(&*code_compressed);

    assert_eq!(
        run(&*code_blc, Input::Bytes(&*b"herp derp")).unwrap(),
        "herp derp"
    );
}
