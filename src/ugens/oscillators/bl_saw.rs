use std::ptr::null_mut;

use crate::ffi::{sp_blsaw, sp_blsaw_compute, sp_blsaw_create, sp_blsaw_destroy, sp_blsaw_init};
use crate::soundpipe::Soundpipe;

pub struct BlSaw {
    sp: Soundpipe,
    ffi: *mut sp_blsaw,
}

unsafe impl Send for BlSaw {}

impl BlSaw {
    pub fn new(sp: Soundpipe) -> Self {
        let mut result = BlSaw {
            sp,
            ffi: null_mut(),
        };
        unsafe {
            sp_blsaw_create(&mut result.ffi);
            sp_blsaw_init(*result.sp.sp_ffi, result.ffi);
        }
        result
    }

    pub fn set_freq(&self, freq: f32) {
        unsafe {
            *(*self.ffi).freq = freq;
        }
    }

    pub fn set_amp(&self, amp: f32) {
        unsafe {
            *(*self.ffi).amp = amp;
        }
    }

    pub fn compute(&self) -> f32 {
        let mut out: f32 = 0.0;
        let null = null_mut();
        unsafe {
            sp_blsaw_compute(*self.sp.sp_ffi, self.ffi, null, &mut out);
        }
        out
    }
}

impl Drop for BlSaw {
    fn drop(&mut self) {
        unsafe {
            sp_blsaw_destroy(&mut self.ffi);
        }
    }
}
