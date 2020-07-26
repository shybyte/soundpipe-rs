use std::ptr::null_mut;

use crate::ffi::{sp_adsr, sp_adsr_compute, sp_adsr_create, sp_adsr_destroy, sp_adsr_init};
use crate::soundpipe::Soundpipe;

pub struct Adsr {
    sp: Soundpipe,
    ffi: *mut sp_adsr,
}

unsafe impl Send for Adsr {}

impl Adsr {
    pub fn new(sp: Soundpipe) -> Self {
        let mut result = Adsr {
            sp,
            ffi: null_mut(),
        };
        unsafe {
            sp_adsr_create(&mut result.ffi);
            sp_adsr_init(*result.sp.sp_ffi, result.ffi);
        }
        result
    }

    pub fn set_attack_time(&self, time_in_seconds: f32) {
        unsafe {
            (*self.ffi).atk = time_in_seconds;
        }
    }

    pub fn set_decay_time(&self, time_in_seconds: f32) {
        unsafe {
            (*self.ffi).dec = time_in_seconds;
        }
    }

    pub fn compute(&self, input: f32) -> f32 {
        let mut out: f32 = 0.0;
        let mut input = input;
        unsafe {
            sp_adsr_compute(*self.sp.sp_ffi, self.ffi, &mut input, &mut out);
        }
        out
    }
}

impl Drop for Adsr {
    fn drop(&mut self) {
        unsafe {
            sp_adsr_destroy(&mut self.ffi);
        }
    }
}
