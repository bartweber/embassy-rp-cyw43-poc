//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

use std::{env, fs};
use std::fs::File;
use std::io::{Write};
use std::path::PathBuf;
use build_const::ConstWriter;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    pub wlan: Wlan,
    pub server: Server,
}

#[derive(Deserialize)]
struct Wlan {
    pub ssid: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize)]
struct Server {
    pub port: Option<u16>
}

fn main() {
    let consts = ConstWriter::for_build("constants").unwrap();

    let mut consts = consts.finish_dependencies();

    let config_raw = fs::read_to_string("config.toml").expect("Failed to read config.toml");
    let config: Config = toml::from_str(&config_raw).expect("Failed to parse config.toml");

    consts.add_value("WLAN_SSID", "&str", &config.wlan.ssid.expect("wlan.ssid not set in config.toml"));
    consts.add_value("WLAN_PASSWORD", "&str", &config.wlan.password.expect("wlan.password not set in config.toml"));
    consts.add_value("SERVER_PORT", "u16", &config.server.port.expect("server.port not set in config.toml"));

    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("../memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");

    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tlink-rp.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
