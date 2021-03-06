/* Copyright (c) Fortanix, Inc.
 *
 * Licensed under the GNU General Public License, version 2 <LICENSE-GPL or
 * https://www.gnu.org/licenses/gpl-2.0.html> or the Apache License, Version
 * 2.0 <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>, at your
 * option. This file may not be copied, modified, or distributed except
 * according to those terms. */

extern crate gcc;

use std::env;

fn main() {
    let mut c = gcc::Config::new();
    c.file("src/rust_printf.c");
    if env::var_os("CARGO_FEATURE_STD").is_none()
        || env::var("TARGET").map(|s| s == "x86_64-unknown-none-gnu") == Ok(true)
    {
        c.flag("-U_FORTIFY_SOURCE")
            .define("_FORTIFY_SOURCE", Some("0"))
            .flag("-ffreestanding");
    }
    c.compile("librust-mbedtls.a");
    // Force correct link order for mbedtls_printf
    println!("cargo:rustc-link-lib=static=mbedtls");
    println!("cargo:rustc-link-lib=static=mbedx509");
    println!("cargo:rustc-link-lib=static=mbedcrypto");
}
