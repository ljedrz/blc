# blc

**blc** is an implementation of the
[binary lambda calculus](https://esolangs.org/wiki/Binary_lambda_calculus).

## Binary lambda calculus basics

Binary lambda calculus (BLC) is a minimal, purely functional programming language based on a binary
encoding of the untyped [lambda calculus](https://en.wikipedia.org/wiki/Lambda_calculus) with
[De Bruijn indices](https://en.wikipedia.org/wiki/De_Bruijn_index).

Lambda terms have the following binary representation in BLC:

| term        | De Bruijn | BLC            |
--------------|-----------|----------------|
| abstraction | λM        | 00M            |
| application | MN        | 01MN           |
| variable    | i         | 1<sup>i</sup>0 |

BLC can operate on bytestrings; a bytestring is encoded as a
[Church list](https://en.wikipedia.org/wiki/Church_encoding#One_pair_as_a_list_node) of lists of
bytes and a byte is encoded as Church list of bits.

Bits 0 and 1 are encoded with
[Church booleans](https://en.wikipedia.org/wiki/Church_encoding#Church_Booleans):

| bit | De Bruijn | BLC     |
|-----|-----------|---------|
| 0   | λλ2       | 0000110 |
| 1   | λλ1       | 000010  |

## [Documentation](https://docs.rs/blc)

## Status

The library is already usable, but it is still a work in progress.

## TODO

- better documentation
- more blc examples
