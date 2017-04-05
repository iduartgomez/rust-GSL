//! N-tuples

use libc::{c_char, size_t, c_void, FILE};

use enums;

extern "C" {
    // Creating ntuples
    pub fn gsl_ntuple_create(filename: *mut c_char,
                             ntuple_data: *mut c_void,
                             size: size_t)
                             -> *mut gsl_ntuple;
    // Opening an existing ntuple file
    pub fn gsl_ntuple_open(filename: *mut c_char,
                           ntuple_data: *mut c_void,
                           size: size_t)
                           -> *mut gsl_ntuple;
    // Writing ntuples
    pub fn gsl_ntuple_write(ntuple: *mut gsl_ntuple) -> enums::Value;
    pub fn gsl_ntuple_bookdata(ntuple: *mut gsl_ntuple) -> enums::Value;
    // Reading ntuples
    pub fn gsl_ntuple_read(ntuple: *mut gsl_ntuple) -> enums::Value;
    // Closing an ntuple file
    pub fn gsl_ntuple_close(ntuple: *mut gsl_ntuple) -> enums::Value;
}

#[repr(C)]
pub struct gsl_ntuple {
    pub file: *mut FILE,
    pub ntuple_data: *mut c_void,
    pub size: size_t,
}
