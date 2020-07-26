#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!("ffi-generated.rs"));

#[cfg(test)]
mod tests {
    use crate::ffi::{sp_blsaw_compute, sp_blsaw_create, sp_blsaw_init, sp_create};
    use std::ptr::null_mut;

    #[test]
    fn it_works() {
        unsafe {
            let mut sp = null_mut();
            sp_create(&mut sp);

            let mut blsaw = null_mut();
            sp_blsaw_create(&mut blsaw);
            sp_blsaw_init(sp, blsaw);
            assert_eq!(*(*blsaw).freq, 440.0);
            (*blsaw).freq = &mut 100.0;
            (*blsaw).amp = &mut 1.0;

            let mut out: f32 = 0.0;

            let null = null_mut();
            sp_blsaw_compute(sp, blsaw, null, &mut out);
            sp_blsaw_compute(sp, blsaw, null, &mut out);

            assert_eq!(out, 0.0);
        }
        assert_eq!(2 + 2, 4);
    }
}
