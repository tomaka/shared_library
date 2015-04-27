#[macro_use]
extern crate shared_library;
extern crate libc;

use std::path::Path;

shared_library!(Test1,
    pub fn hello() -> libc::c_int,
    fn hello2(),
);

shared_library!(Test2, "libtest.dll",
    pub fn hello() -> libc::c_int,
    fn hello2(),

    static CONSTANT: &'static u32,
);

fn main() {
    unsafe { hello() };

    let test1 = Test1::open(Path::new("libtest.dll")).unwrap();
    unsafe { (test1.hello)() };
    unsafe { (test1.hello2)() };
}
