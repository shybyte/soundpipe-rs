use crate::ffi::{sp_create, sp_midi2cps};
use crate::ffi::{sp_data, sp_destroy};
use std::ptr::null_mut;
use std::rc::Rc;

#[derive(Clone)]
pub struct Soundpipe {
    pub(crate) sp_ffi: Rc<*mut sp_data>,
}

unsafe impl Send for Soundpipe {}

impl Soundpipe {
    pub fn new(sample_rate: i32) -> Self {
        let mut sp = null_mut();
        unsafe {
            sp_create(&mut sp);
            (*sp).sr = sample_rate;
        }
        Soundpipe {
            sp_ffi: Rc::new(sp),
        }
    }
}

impl Drop for Soundpipe {
    fn drop(&mut self) {
        if let Some(sp) = Rc::get_mut(&mut self.sp_ffi) {
            unsafe {
                sp_destroy(sp);
            }
        }
    }
}

pub fn midi2cps(midi_note: f32) -> f32 {
    unsafe { sp_midi2cps(midi_note) }
}
