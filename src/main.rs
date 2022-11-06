use std::ffi::CString;
use std::os::raw::c_char;

fn main() {
    let pattern = CString::new("Hello.*").unwrap();
    let input = CString::new("Hello World").unwrap();
    let matches = match unsafe { is_match(pattern.as_ptr(), input.as_ptr()) } {
        true => "matches",
        false => "does not match",
    };
    println!("{pattern:?} {matches} the input {input:?}");
}

extern "C" {
    fn is_match(pattern: *const c_char, string: *const c_char) -> bool;
}
