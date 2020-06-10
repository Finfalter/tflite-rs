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
    pub(crate) handle: Box<bindings::TfLiteExternalContext>,
}

impl Drop for ExternalContext {
    fn drop(&mut self) {
        let handle = Box::into_raw(mem::take(&mut self.handle));

        #[allow(clippy::forget_copy, clippy::useless_transmute, deprecated)]
        unsafe {
            cpp!([handle as "TfLiteExternalContext*"] {
                delete handle;
            });
        }
    }
}

impl ExternalContext {
    pub(crate) fn handle_mut(&mut self) -> &mut bindings::TfLiteExternalContext {
        use std::ops::DerefMut;
        self.handle.deref_mut()
    }

    pub unsafe fn from_raw(raw: *mut bindings::TfLiteExternalContext) -> Self {
        let handle = Box::from_raw(raw);
        Self { handle }
    }
}
