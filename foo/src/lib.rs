extern crate bar;

mod localmodule;

use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn foo(u: c_char ){
    println!("Foo from rust!");
    println!("Foo from rust 2");

    let t = bar::bar(u as u8);
    let t2 = localbar(t as i8);
    let t3 = localmodule::fn_in_module(t2 as u8);
    println!("bar={} t2={} t3={}", t, t2, t3);
}


fn localbar(u: i8) -> u16 {
    u as u16 & 7
}