use std::str;
use std::str::{FromStr, Utf8Error};

pub(crate) fn concat_slice_vec(c: &[u8], done: Vec<u8>) -> Vec<u8> {
    let mut new_vec = c.to_vec();
    new_vec.extend(&done);
    new_vec
}

pub(crate) fn convert_vec_utf8(v: Vec<u8>) -> Result<String, Utf8Error> {
    let slice = v.as_slice();
    str::from_utf8(slice).map(|s| s.to_owned())
}

pub(crate) fn complete_byte_slice_str_from_utf8(c: &[u8]) -> Result<&str, Utf8Error> {
    str::from_utf8(c)
}

pub(crate) fn complete_str_from_str<F: FromStr>(c: &str) -> Result<F, F::Err> {
    FromStr::from_str(c)
}