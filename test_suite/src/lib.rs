#![no_std]
#![cfg_attr(feature = "unstable", feature(never_type))]
#![cfg_attr(feature = "unstable", feature(non_ascii_idents))]

#[macro_use]
extern crate sgx_tstd as std;

mod bytes;
#[macro_use]
mod macros;
mod test_annotations;
mod test_borrow;
mod test_de;
mod test_gen;
mod test_identifier;
mod test_ignored_any;
mod test_macros;
mod test_remote;
mod test_roundtrip;
mod test_ser;
mod test_serde_path;
mod test_value;
mod unstable;

pub mod tests {
    use testing::generate_runner;

    generate_runner!();
}
