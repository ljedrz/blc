extern crate blc;

use blc::*;
use blc::encoding::binary::decompress;
use blc::execution::Input;

#[test]
// program code from http://www.ioccc.org/2012/tromp/inflate.Blc
fn inflate() {
    let code_compressed =
        [0x44, 0x44, 0x68, 0x16, 0x01, 0x79, 0x1a, 0x00, 0x16, 0x7f, 0xfb, 0xcb, 0xcf, 0xdf,
         0x65, 0xfb, 0xed, 0x0f, 0x3c, 0xe7, 0x3c, 0xf3, 0xc2, 0xd8, 0x20, 0x58, 0x2c, 0x0b,
         0x06, 0xc0];
    let code_blc = decompress(&code_compressed);

    assert_eq!(
        run(&*code_blc, Input::Bytes(&[0x1, 0x7a, 0x74])).unwrap(),
        "000000010111101001110100"
    );
}