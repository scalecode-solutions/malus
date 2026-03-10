//! CGDataProvider — read-only access to a block of data.

use crate::core_foundation::base::{CFRef, CFTypeRef};
use crate::core_foundation::data::CFDataRef;
use std::ffi::c_void;
use std::fmt;

#[repr(C)]
pub struct __CGDataProvider(c_void);
pub type CGDataProviderRef = *const __CGDataProvider;

type ReleaseCallback = unsafe extern "C" fn(info: *mut c_void, data: *const c_void, size: usize);

extern "C" {
    fn CGDataProviderCreateWithData(
        info: *mut c_void,
        data: *const c_void,
        size: usize,
        release_data: Option<ReleaseCallback>,
    ) -> CGDataProviderRef;

    fn CGDataProviderCreateWithCFData(data: CFDataRef) -> CGDataProviderRef;
}

// ============================================================================
// CGDataProvider newtype
// ============================================================================

pub struct CGDataProvider(CFRef<__CGDataProvider>);

impl CGDataProvider {
    /// Create from a CFData (copies/retains the data).
    pub fn from_cf_data(data: &crate::core_foundation::CFData) -> Self {
        unsafe {
            let raw = CGDataProviderCreateWithCFData(data.as_raw());
            Self(CFRef::wrap(raw).expect("CGDataProviderCreateWithCFData returned null"))
        }
    }

    /// Create from a byte slice. The data is copied into a Box and freed
    /// when the provider is released.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let owned = bytes.to_vec().into_boxed_slice();
        let len = owned.len();
        let ptr = Box::into_raw(owned) as *const c_void;

        unsafe extern "C" fn release_boxed_slice(
            _info: *mut c_void,
            data: *const c_void,
            size: usize,
        ) {
            let _ = Vec::from_raw_parts(data as *mut u8, size, size);
        }

        unsafe {
            let raw = CGDataProviderCreateWithData(
                std::ptr::null_mut(),
                ptr,
                len,
                Some(release_boxed_slice),
            );
            Self(CFRef::wrap(raw).expect("CGDataProviderCreateWithData returned null"))
        }
    }

    #[inline]
    pub unsafe fn wrap(ptr: CGDataProviderRef) -> Option<Self> {
        CFRef::wrap(ptr).map(Self)
    }

    #[inline]
    pub unsafe fn retain(ptr: CGDataProviderRef) -> Option<Self> {
        CFRef::retain(ptr).map(Self)
    }

    #[inline]
    pub fn as_raw(&self) -> CGDataProviderRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Clone for CGDataProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl fmt::Debug for CGDataProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CGDataProvider({:?})", self.0)
    }
}
