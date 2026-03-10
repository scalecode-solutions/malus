//! NSApplication — the application singleton and event loop.

use crate::runtime::*;

use super::event::NSEvent;

/// Activation policy for the application.
#[repr(isize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ActivationPolicy {
    Regular = 0,
    Accessory = 1,
    Prohibited = 2,
}

/// The application singleton.
pub struct NSApplication(pub(super) Id);

impl NSApplication {
    /// Return the raw ObjC pointer (does not transfer ownership).
    pub fn as_raw(&self) -> Id {
        self.0
    }

    /// Get the shared application instance.
    ///
    /// This retains the returned singleton so the wrapper owns a +1 reference.
    pub fn shared() -> Self {
        unsafe {
            let raw: Id = msg_send!(
                cls!("NSApplication") as Id,
                sel!("sharedApplication"),
                fn(Id, Sel) -> Id
            );
            Self(retain(raw))
        }
    }

    /// Set the activation policy.
    pub fn set_activation_policy(&self, policy: ActivationPolicy) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setActivationPolicy:"),
                fn(Id, Sel, NSInteger) -> (),
                policy as NSInteger
            );
        }
    }

    /// Activate the application, optionally ignoring other apps.
    pub fn activate_ignoring_other_apps(&self, ignore: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("activateIgnoringOtherApps:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(ignore)
            );
        }
    }

    /// Run the event loop. This never returns; exit via `std::process::exit`.
    pub fn run(self) -> ! {
        unsafe {
            msg_send!(self.0, sel!("run"), fn(Id, Sel) -> ());
        }
        // run never returns, but satisfy the type system
        unreachable!()
    }

    /// Set the application's main menu.
    pub fn set_main_menu(&self, menu: &super::menu::NSMenu) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setMainMenu:"),
                fn(Id, Sel, Id) -> (),
                menu.as_raw()
            );
        }
    }

    /// Terminate the application.
    pub fn terminate(sender: Id) {
        unsafe {
            let app: Id = msg_send!(
                cls!("NSApplication") as Id,
                sel!("sharedApplication"),
                fn(Id, Sel) -> Id
            );
            msg_send!(
                app,
                sel!("terminate:"),
                fn(Id, Sel, Id) -> (),
                sender
            );
        }
    }

    /// Get the current event, if any.
    pub fn current_event(&self) -> Option<NSEvent> {
        unsafe {
            let raw: Id = msg_send!(
                self.0,
                sel!("currentEvent"),
                fn(Id, Sel) -> Id
            );
            if raw.is_null() {
                None
            } else {
                Some(NSEvent(retain(raw)))
            }
        }
    }
}

impl Clone for NSApplication {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSApplication {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
