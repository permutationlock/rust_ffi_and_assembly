extern crate libc;

#[link(name = "fooc")]
extern "C" {
    fn fooc(input: libc::c_int) -> libc::c_int;
}

#[link(name = "fooasm")]
extern "C" {
    fn fooasm(input: libc::c_int) -> libc::c_int;
}

fn main() {
    let msg = std::ffi::CString::new(
        "c returned %d, asm returned %d\n"
    ).unwrap();
    unsafe {
        libc::printf(msg.as_ptr(), fooc(7), fooasm(5));
    }
}
