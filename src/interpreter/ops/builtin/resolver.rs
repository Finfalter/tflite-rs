use std::mem;

use crate::bindings::tflite as bindings;
use crate::interpreter::op_resolver::{OpResolver, Registration};

cpp! {{
    #include "tensorflow/lite/kernels/register.h"

    using namespace tflite::ops::builtin;
}}

pub struct Resolver {
    handle: Box<bindings::OpResolver>,
}

impl Drop for Resolver {
    #[allow(clippy::useless_transmute, clippy::forget_copy, deprecated)]
    fn drop(&mut self) {
        let handle = Box::into_raw(mem::take(&mut self.handle));
        unsafe {
            cpp!([handle as "BuiltinOpResolver*"] {
                delete handle;
            });
        }
    }
}

impl OpResolver for Resolver {
    fn get_resolver_handle(&self) -> &bindings::OpResolver {
        self.handle.as_ref()
    }

    fn add_custom<T: AsRef<str>>(&self, name: T, registration: &Registration) {
        use std::ops::Deref;
        let name_cstr = std::ffi::CString::new(name.as_ref()).unwrap();
        let name = name_cstr.as_ptr();
        let resolver = self.handle.deref();

        #[allow(clippy::forget_copy, clippy::useless_transmute, deprecated)]
        unsafe {
            cpp!([resolver as "BuiltinOpResolver*", name as "const char*", registration as "const TfLiteRegistration*"] {
                resolver->AddCustom(name, registration);
            })
        };
    }
}

impl Default for Resolver {
    #[allow(clippy::forget_copy, deprecated)]
    fn default() -> Self {
        let handle = unsafe {
            cpp!([] -> *mut bindings::OpResolver as "OpResolver*" {
                return new BuiltinOpResolver();
            })
        };
        let handle = unsafe { Box::from_raw(handle) };
        Self { handle }
    }
}
