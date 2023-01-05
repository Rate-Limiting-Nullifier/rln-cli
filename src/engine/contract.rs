use fs_extra::dir::{copy, CopyOptions};

pub fn contract() {
    let options = CopyOptions::new();

    copy("./resources/vendor/rln-contract", "./", &options).unwrap();
}
