extern crate blc;
extern crate lambda_calculus;

use blc::*;
use blc::encoding::lambda::encode;
use blc::execution::Input;
use lambda_calculus::*;
use lambda_calculus::data::num::church::{is_zero, rem};

#[test]
fn fizz_buzz() {
    let fizzbuzz_single =
        abs(
            app!(
                is_zero(),
                app!(rem(), Var(1), 15.into_church()),
                encode(&*b"FizzBuzz"),
                app!(
                    is_zero(),
                    app!(rem(), Var(1), 3.into_church()),
                    encode(&*b"Fizz"),
                    app!(
                        is_zero(),
                        app!(rem(), Var(1), 5.into_church()),
                        encode(&*b"Buzz"),
                        Var(1)
                    )
                )
            )
        );
    let fizzbuzz_blc = to_bits(&fizzbuzz_single);

    assert_eq!(
        run(&*fizzbuzz_blc, Input::Bits(&to_bits(&1.into_church()))).unwrap(),
        "(λλ21)" // Church-encoded 1
    );

    assert_eq!(
        run(&*fizzbuzz_blc, Input::Bits(&to_bits(&2.into_church()))).unwrap(),
        "(λλ2(21))" // Church-encoded 2
    );
    assert_eq!(
        run(&*fizzbuzz_blc, Input::Bits(&to_bits(&3.into_church()))).unwrap(),
        "Fizz"
    );

    assert_eq!(
        run(&*fizzbuzz_blc, Input::Bits(&to_bits(&4.into_church()))).unwrap(),
        "(λλ2(2(2(21))))" // Church-encoded 4
    );

    assert_eq!(
        run(&*fizzbuzz_blc, Input::Bits(&to_bits(&5.into_church()))).unwrap(),
        "Buzz"
    );

    assert_eq!(
        run(&*fizzbuzz_blc, Input::Bits(&to_bits(&15.into_church()))).unwrap(),
        "FizzBuzz"
    );
}
