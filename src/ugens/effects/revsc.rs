use soundpipe_derive::UGenMacro;

use crate::ffi::{sp_revsc, sp_revsc_compute, sp_revsc_create, sp_revsc_destroy, sp_revsc_init};
use crate::soundpipe::Soundpipe;

#[derive(UGenMacro)]
pub struct Revsc {
    sp: Soundpipe,
    ffi: *mut sp_revsc,
}

impl Revsc {
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

    pub fn compute_internal(
        &self,
        in_left: &mut f32,
        in_right: &mut f32,
        out_left: &mut f32,
        out_right: &mut f32,
    ) {
        unsafe {
            sp_revsc_compute(
                *self.sp.sp_ffi,
                self.ffi,
                in_left,
                in_right,
                out_left,
                out_right,
            );
        }
    }

    pub fn compute(&self, in_left: f32, in_right: f32) -> (f32, f32) {
        let mut in_left: f32 = in_left;
        let mut in_right: f32 = in_right;
        let mut out_left: f32 = 0.0;
        let mut out_right: f32 = 0.0;
        self.compute_internal(&mut in_left, &mut in_right, &mut out_left, &mut out_right);
        (out_left, out_right)
    }
}
