use std::ptr::null_mut;

use crate::ffi::{sp_revsc, sp_revsc_compute, sp_revsc_create, sp_revsc_destroy, sp_revsc_init};
use crate::soundpipe::Soundpipe;

pub struct Revsc {
    sp: Soundpipe,
    ffi: *mut sp_revsc,
}

unsafe impl Send for Revsc {}

impl Revsc {
    pub fn new(sp: Soundpipe) -> Self {
        let mut result = Revsc {
            sp,
            ffi: null_mut(),
        };
        unsafe {
            sp_revsc_create(&mut result.ffi);
            sp_revsc_init(*result.sp.sp_ffi, result.ffi);
        }
        result
    }

    pub fn set_feedback(&self, feedback: f32) {
        unsafe {
            (*self.ffi).feedback = feedback;
        }
    }

    pub fn set_lpfreq(&self, lpfreq: f32) {
        unsafe {
            (*self.ffi).lpfreq = lpfreq;
        }
    }

    pub fn compute(&self, in_left: f32, in_right: f32) -> (f32, f32) {
        let mut in_left: f32 = in_left;
        let mut in_right: f32 = in_right;
        let mut out_left: f32 = 0.0;
        let mut out_right: f32 = 0.0;
        unsafe {
            sp_revsc_compute(*self.sp.sp_ffi, self.ffi, &mut in_left, &mut in_right, &mut out_left, &mut out_right);
        }
        (out_left, out_right)
    }
}

impl Drop for Revsc {
    fn drop(&mut self) {
        unsafe {
            sp_revsc_destroy(&mut self.ffi);
        }
    }
}
