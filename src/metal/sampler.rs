//! MTLSamplerDescriptor and MTLSamplerState.

use crate::runtime::*;
use super::device::MTLDevice;
use super::types::{MTLSamplerAddressMode, MTLSamplerMinMagFilter, MTLSamplerMipFilter};

// ============================================================================
// MTLSamplerDescriptor
// ============================================================================

pub struct MTLSamplerDescriptor(Id);

impl MTLSamplerDescriptor {
    /// Create a new sampler descriptor with default settings.
    pub fn new() -> Self {
        unsafe {
            let raw = alloc_init(cls!("MTLSamplerDescriptor") as Id);
            assert!(!raw.is_null(), "failed to create MTLSamplerDescriptor");
            Self(raw)
        }
    }

    pub fn set_min_filter(&self, filter: MTLSamplerMinMagFilter) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setMinFilter:"),
                fn(Id, Sel, NSUInteger) -> (),
                filter as NSUInteger
            );
        }
    }

    pub fn set_mag_filter(&self, filter: MTLSamplerMinMagFilter) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setMagFilter:"),
                fn(Id, Sel, NSUInteger) -> (),
                filter as NSUInteger
            );
        }
    }

    pub fn set_mip_filter(&self, filter: MTLSamplerMipFilter) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setMipFilter:"),
                fn(Id, Sel, NSUInteger) -> (),
                filter as NSUInteger
            );
        }
    }

    pub fn set_s_address_mode(&self, mode: MTLSamplerAddressMode) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setSAddressMode:"),
                fn(Id, Sel, NSUInteger) -> (),
                mode as NSUInteger
            );
        }
    }

    pub fn set_t_address_mode(&self, mode: MTLSamplerAddressMode) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setTAddressMode:"),
                fn(Id, Sel, NSUInteger) -> (),
                mode as NSUInteger
            );
        }
    }

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

impl Clone for MTLSamplerDescriptor {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for MTLSamplerDescriptor {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

impl Default for MTLSamplerDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// MTLSamplerState
// ============================================================================

pub struct MTLSamplerState(Id);

impl MTLSamplerState {
    /// Create a sampler state from a device and descriptor.
    pub fn new(device: &MTLDevice, descriptor: &MTLSamplerDescriptor) -> Self {
        unsafe {
            let raw: Id = msg_send!(
                device.as_raw(),
                sel!("newSamplerStateWithDescriptor:"),
                fn(Id, Sel, Id) -> Id,
                descriptor.as_raw()
            );
            assert!(!raw.is_null(), "newSamplerStateWithDescriptor returned null");
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

impl Clone for MTLSamplerState {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for MTLSamplerState {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

unsafe impl Send for MTLSamplerState {}
unsafe impl Sync for MTLSamplerState {}
