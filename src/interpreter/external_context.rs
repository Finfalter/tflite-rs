use std::mem;

use crate::bindings;

cpp! {{
    using namespace tflite;
}}

pub enum ExternalContextType {
    Eigen,
    GemmLowp,
    EdgeTpu,
    CpuBackend,
    MaxExternal,
}

#[derive(Default)]
pub struct ExternalContext {
    pub(crate) handle: Option<Box<bindings::TfLiteExternalContext>>,
}

impl Drop for ExternalContext {
    fn drop(&mut self) {
        if self.handle.is_some() {
            let mut handle = self.handle.take().unwrap();
            let handle = Box::into_raw(mem::take(&mut handle));

            #[allow(clippy::forget_copy, clippy::useless_transmute, deprecated)]
            unsafe {
                cpp!([handle as "TfLiteExternalContext*"] {
                    delete handle;
                });
            }
        }
    }
}

impl ExternalContext {
    pub(crate) fn into_raw(mut self) -> *mut bindings::TfLiteExternalContext {
        let handle = Box::into_raw(mem::take(&mut self.handle.take().unwrap()));
        return handle;
    }

    pub unsafe fn from_raw(raw: *mut bindings::TfLiteExternalContext) -> Self {
        let handle = Box::from_raw(raw);
        Self { handle: Some(handle) }
    }
}
