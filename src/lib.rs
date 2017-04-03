// extern crate libcruby_sys as sys;

use std::ffi::CString;

type VALUE = *mut ();

extern "C" {
    // #[link_name = "HELIX_Qnil"]
    // static Qnil: VALUE;
    #[link_name = "rb_cObject"]
    static rb_cObject: VALUE;
    #[link_name = "rb_eRuntimeError"]
    static rb_eRuntimeError: VALUE;

    fn rb_define_method(class: VALUE,
                        name: *const i8,
                        func: *const (),
                        arity: isize);
    fn rb_raise(exc: VALUE,
                string: *const i8,
                ...);
}

// This will become Object#panic in Ruby
#[no_mangle]
pub extern "C" fn panic() -> VALUE {
    let _ = ::std::panic::catch_unwind(|| {
        panic!("Panic!")
    });
    rust_raise();
    unreachable!();
}

// This will become Object#rust_raise in Ruby
#[no_mangle]
pub extern "C" fn rust_raise() {
    unsafe {
        rb_raise(rb_eRuntimeError,
                 CString::new("Panicked in Rust").unwrap().as_ptr())
    }
}

#[no_mangle]
pub extern "C" fn panic_and_raise() {
    {
        let _ = panic();
    }
    rust_raise()
}

// This method is called when Ruby loads the native extension
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_libpanic_test() {
    unsafe {
        // This is a built-in Ruby method that allows us to define C methods on Ruby object
        rb_define_method(rb_cObject,
                         CString::new("panic").unwrap().as_ptr(),
                         panic as *const _,
                         0);
        rb_define_method(rb_cObject,
                         CString::new("rust_raise").unwrap().as_ptr(),
                         rust_raise as *const _,
                         0);
        rb_define_method(rb_cObject,
                         CString::new("panic_and_raise").unwrap().as_ptr(),
                         panic_and_raise as *const _,
                         0);
    }
}
