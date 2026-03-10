//! CGDisplay — screen info and configuration.

use super::geometry::{CGFloat, CGRect};

pub type CGDirectDisplayID = u32;

extern "C" {
    fn CGMainDisplayID() -> CGDirectDisplayID;
    fn CGDisplayPixelsWide(display: CGDirectDisplayID) -> usize;
    fn CGDisplayPixelsHigh(display: CGDirectDisplayID) -> usize;
    fn CGDisplayBounds(display: CGDirectDisplayID) -> CGRect;
    fn CGDisplayScreenSize(display: CGDirectDisplayID) -> super::geometry::CGSize;
    fn CGGetActiveDisplayList(
        max_displays: u32,
        active_displays: *mut CGDirectDisplayID,
        display_count: *mut u32,
    ) -> i32; // CGError
}

// ============================================================================
// CGDisplay — value type (just wraps the display ID)
// ============================================================================

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CGDisplay(pub CGDirectDisplayID);

impl CGDisplay {
    pub fn main() -> Self {
        Self(unsafe { CGMainDisplayID() })
    }

    /// List all active displays.
    pub fn active_displays() -> Vec<Self> {
        unsafe {
            let mut count: u32 = 0;
            CGGetActiveDisplayList(0, std::ptr::null_mut(), &mut count);
            if count == 0 {
                return Vec::new();
            }
            let mut ids = vec![0u32; count as usize];
            CGGetActiveDisplayList(count, ids.as_mut_ptr(), &mut count);
            ids.truncate(count as usize);
            ids.into_iter().map(Self).collect()
        }
    }

    pub fn id(&self) -> CGDirectDisplayID {
        self.0
    }

    /// Width in pixels.
    pub fn pixels_wide(&self) -> usize {
        unsafe { CGDisplayPixelsWide(self.0) }
    }

    /// Height in pixels.
    pub fn pixels_high(&self) -> usize {
        unsafe { CGDisplayPixelsHigh(self.0) }
    }

    /// Bounds in the global display coordinate space.
    pub fn bounds(&self) -> CGRect {
        unsafe { CGDisplayBounds(self.0) }
    }

    /// Physical screen size in millimeters.
    pub fn screen_size_mm(&self) -> (CGFloat, CGFloat) {
        let size = unsafe { CGDisplayScreenSize(self.0) };
        (size.width, size.height)
    }
}
