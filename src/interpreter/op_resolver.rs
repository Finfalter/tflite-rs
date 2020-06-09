use std::sync::Arc;

use crate::bindings::tflite::OpResolver as SysOpResolver;

pub type Registration = crate::bindings::TfLiteRegistration;

pub trait OpResolver: Send + Sync {
    fn get_resolver_handle(&self) -> &SysOpResolver;
    fn add_custom<T: AsRef<str>>(&self, name: T, registration: &Registration);
}

impl<T: OpResolver> OpResolver for Arc<T> {
    fn get_resolver_handle(&self) -> &SysOpResolver {
        self.as_ref().get_resolver_handle()
    }

    fn add_custom<U: AsRef<str>>(&self, name: U, registration: &Registration) {
        self.as_ref().add_custom(name, registration)
    }
}

impl<'a, T: OpResolver> OpResolver for &'a T {
    fn get_resolver_handle(&self) -> &SysOpResolver {
        (*self).get_resolver_handle()
    }

    fn add_custom<U: AsRef<str>>(&self, name: U, registration: &Registration) {
        (*self).add_custom(name, registration)
    }
}
