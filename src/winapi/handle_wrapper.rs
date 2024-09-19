use std::{borrow::Borrow, cell::RefCell, ptr::null_mut, rc::Rc};

use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};

pub struct Handle(HANDLE);
impl Handle {
    pub fn new() -> Handle {
        Self(null_mut())
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { CloseHandle(self.0) };
        }
    }
}

impl From<HANDLE> for Handle {
    fn from(value: HANDLE) -> Self {
        Self(value)
    }
}

#[derive(Clone)]
pub struct SmartHandle(Rc<RefCell<Handle>>);

impl SmartHandle {
    pub fn new() -> SmartHandle {
        Self(Rc::new(RefCell::new(Handle(null_mut()))))
    }

    pub fn get_raw(&self) -> HANDLE {
        let x: &RefCell<Handle> = self.0.borrow();
        x.borrow().0
    }
}

impl From<Handle> for SmartHandle {
    fn from(value: Handle) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }
}

impl From<HANDLE> for SmartHandle {
    fn from(value: HANDLE) -> Self {
        Self(Rc::new(RefCell::new(Handle(value))))
    }
}
