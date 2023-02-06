use fs_extra::dir::{copy, CopyOptions};

use minijinja::render;

pub fn contract(depth: u8, deposit: u128) {
    let options = CopyOptions::new();

    copy("./resources/vendor/rln-contract", "./", &options).unwrap();

    let rln_file = fs_extra::file::read_to_string("./rln-contract/contracts/Rln.sol").unwrap();
    let rln_file = render!(&rln_file, membership_deposit => deposit, depth => depth);

    fs_extra::file::write_all("./rln-contract/contracts/Rln.sol", &rln_file).unwrap();
}
