//! NSAppearance — visual appearance theming.

use crate::runtime::*;

/// An appearance object.
pub struct NSAppearance(pub(super) Id);

impl NSAppearance {
    /// Return the raw ObjC pointer (does not transfer ownership).
    pub fn as_raw(&self) -> Id {
        self.0
    }

    /// Look up an appearance by name. Returns `None` if not found.
    pub fn named(name: &str) -> Option<Self> {
        unsafe {
            let ns = nsstring(name);
            let raw: Id = msg_send!(
                cls!("NSAppearance") as Id,
                sel!("appearanceNamed:"),
                fn(Id, Sel, Id) -> Id,
                ns
            );
            if raw.is_null() {
                None
            } else {
                Some(Self(retain(raw)))
            }
        }
    }

    /// The standard Aqua (light) appearance.
    pub fn aqua() -> Self {
        Self::named("NSAppearanceNameAqua").expect("NSAppearanceNameAqua not found")
    }

    /// The Dark Aqua appearance.
    pub fn dark_aqua() -> Self {
        Self::named("NSAppearanceNameDarkAqua").expect("NSAppearanceNameDarkAqua not found")
    }

    /// The vibrant light appearance.
    pub fn vibrant_light() -> Self {
        Self::named("NSAppearanceNameVibrantLight").expect("NSAppearanceNameVibrantLight not found")
    }

    /// The vibrant dark appearance.
    pub fn vibrant_dark() -> Self {
        Self::named("NSAppearanceNameVibrantDark").expect("NSAppearanceNameVibrantDark not found")
    }

    /// Get the appearance name as a string.
    pub fn name(&self) -> String {
        unsafe {
            let ns: Id = msg_send!(self.0, sel!("name"), fn(Id, Sel) -> Id);
            from_nsstring(ns)
        }
    }
}

impl Clone for NSAppearance {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSAppearance {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
