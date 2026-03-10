//! CTFontManager — register and unregister fonts.

use crate::core_foundation::base::{__CFURL, Boolean, CFTypeRef};

pub type CFURLRef = *const __CFURL;

/// Scope for font registration.
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CTFontManagerScope {
    /// Not registered.
    None = 0,
    /// Available to the calling process only.
    Process = 1,
    /// Available to the current user session.
    Persistent = 2,
}

extern "C" {
    fn CTFontManagerRegisterFontsForURL(
        font_url: CFURLRef,
        scope: CTFontManagerScope,
        error: *mut CFTypeRef,
    ) -> Boolean;

    fn CTFontManagerUnregisterFontsForURL(
        font_url: CFURLRef,
        scope: CTFontManagerScope,
        error: *mut CFTypeRef,
    ) -> Boolean;
}

/// Register fonts from a URL (file path to .ttf/.otf/.ttc).
/// Returns `Ok(())` on success, or `Err(())` on failure.
pub fn register_fonts_for_url(url: CFURLRef, scope: CTFontManagerScope) -> Result<(), ()> {
    unsafe {
        let mut error: CFTypeRef = std::ptr::null();
        let ok = CTFontManagerRegisterFontsForURL(url, scope, &mut error);
        if ok != 0 {
            Ok(())
        } else {
            // Release the error if non-null
            if !error.is_null() {
                crate::core_foundation::base::CFRelease(error);
            }
            Err(())
        }
    }
}

/// Unregister fonts previously registered from a URL.
/// Returns `Ok(())` on success, or `Err(())` on failure.
pub fn unregister_fonts_for_url(url: CFURLRef, scope: CTFontManagerScope) -> Result<(), ()> {
    unsafe {
        let mut error: CFTypeRef = std::ptr::null();
        let ok = CTFontManagerUnregisterFontsForURL(url, scope, &mut error);
        if ok != 0 {
            Ok(())
        } else {
            if !error.is_null() {
                crate::core_foundation::base::CFRelease(error);
            }
            Err(())
        }
    }
}
