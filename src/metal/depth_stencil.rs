//! MTLDepthStencilDescriptor and MTLDepthStencilState.

use crate::runtime::*;
use super::device::MTLDevice;
use super::types::MTLCompareFunction;

// ============================================================================
// MTLDepthStencilDescriptor
// ============================================================================

pub struct MTLDepthStencilDescriptor(Id);

impl MTLDepthStencilDescriptor {
    /// Create a new depth-stencil descriptor.
    pub fn new() -> Self {
        unsafe {
            let raw = alloc_init(cls!("MTLDepthStencilDescriptor") as Id);
            assert!(!raw.is_null(), "failed to create MTLDepthStencilDescriptor");
            Self(raw)
        }
    }

    /// Set the depth compare function.
    pub fn set_depth_compare_function(&self, func: MTLCompareFunction) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setDepthCompareFunction:"),
                fn(Id, Sel, NSUInteger) -> (),
                func as NSUInteger
            );
        }
    }

    /// Set whether depth writes are enabled.
    pub fn set_depth_write_enabled(&self, enabled: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setDepthWriteEnabled:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(enabled)
            );
        }
    }

    /// Set the label.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_str = nsstring(label);
            msg_send!(self.0, sel!("setLabel:"), fn(Id, Sel, Id) -> (), ns_str);
        }
    }

    #[inline]
    pub fn as_raw(&self) -> Id {
        self.0
    }
}

impl Clone for MTLDepthStencilDescriptor {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for MTLDepthStencilDescriptor {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

impl Default for MTLDepthStencilDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// MTLDepthStencilState
// ============================================================================

pub struct MTLDepthStencilState(Id);

impl MTLDepthStencilState {
    /// Create a depth-stencil state from a device and descriptor.
    pub fn new(device: &MTLDevice, descriptor: &MTLDepthStencilDescriptor) -> Self {
        unsafe {
            let raw: Id = msg_send!(
                device.as_raw(),
                sel!("newDepthStencilStateWithDescriptor:"),
                fn(Id, Sel, Id) -> Id,
                descriptor.as_raw()
            );
            assert!(!raw.is_null(), "newDepthStencilStateWithDescriptor returned null");
            Self(raw)
        }
    }

    /// Get the label.
    pub fn label(&self) -> String {
        unsafe {
            let ns_str: Id = msg_send!(self.0, sel!("label"), fn(Id, Sel) -> Id);
            from_nsstring(ns_str)
        }
    }

    #[inline]
    pub fn as_raw(&self) -> Id {
        self.0
    }
}

impl Clone for MTLDepthStencilState {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for MTLDepthStencilState {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

unsafe impl Send for MTLDepthStencilState {}
unsafe impl Sync for MTLDepthStencilState {}
