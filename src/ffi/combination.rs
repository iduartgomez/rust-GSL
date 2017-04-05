//! Combination

use libc::size_t;

use enums;

extern "C" {
    // Combination allocation
    pub fn gsl_combination_alloc(n: size_t, k: size_t) -> *mut gsl_combination;
    pub fn gsl_combination_calloc(n: size_t, k: size_t) -> *mut gsl_combination;
    pub fn gsl_combination_init_first(c: *mut gsl_combination);
    pub fn gsl_combination_init_last(c: *mut gsl_combination);
    pub fn gsl_combination_free(c: *mut gsl_combination);
    pub fn gsl_combination_memcpy(dest: *mut gsl_combination,
                                  src: *const gsl_combination)
                                  -> enums::Value;
    // Accessing combination elements
    pub fn gsl_combination_get(c: *const gsl_combination, i: size_t) -> size_t;
    // Combination properties
    pub fn gsl_combination_n(c: *const gsl_combination) -> size_t;
    pub fn gsl_combination_k(c: *const gsl_combination) -> size_t;
    pub fn gsl_combination_valid(c: *mut gsl_combination) -> enums::Value;
    // Combination functions
    pub fn gsl_combination_next(c: *mut gsl_combination) -> enums::Value;
    pub fn gsl_combination_prev(c: *mut gsl_combination) -> enums::Value;
}

#[repr(C)]
pub struct gsl_combination {
    pub n: size_t,
    pub k: size_t,
    pub data: *mut size_t,
}
