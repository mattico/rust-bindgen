#![crate_name = "bindgen_plugin"]
#![crate_type = "dylib"]
#![feature(plugin_registrar, rustc_private)]

extern crate bindgen;
extern crate rustc_plugin;
extern crate syntax;
extern crate clang_sys;

mod bgmacro;

use rustc_plugin::Registry;

#[doc(hidden)]
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("bindgen", bgmacro::bindgen_macro);
}
