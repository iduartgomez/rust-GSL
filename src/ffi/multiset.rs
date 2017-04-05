//! Multisets

use libc::size_t;

use enums;

extern "C" {
    // Multiset allocation
    pub fn gsl_multiset_alloc(n: size_t, k: size_t) -> *mut gsl_multiset;
    pub fn gsl_multiset_calloc(n: size_t, k: size_t) -> *mut gsl_multiset;
    pub fn gsl_multiset_init_first(c: *mut gsl_multiset);
    pub fn gsl_multiset_init_last(c: *mut gsl_multiset);
    pub fn gsl_multiset_free(c: *mut gsl_multiset);
    pub fn gsl_multiset_memcpy(dest: *mut gsl_multiset, src: *const gsl_multiset) -> enums::Value;
    // Accessing multiset elements
    pub fn gsl_multiset_get(c: *const gsl_multiset, i: size_t) -> size_t;
    // Multiset properties
    pub fn gsl_multiset_n(c: *const gsl_multiset) -> size_t;
    pub fn gsl_multiset_k(c: *const gsl_multiset) -> size_t;
    //pub fn gsl_multiset_data(c: *const gsl_multiset) -> *mut size_t;
    pub fn gsl_multiset_valid(c: *mut gsl_multiset) -> enums::Value;
    // Multiset functions
    pub fn gsl_multiset_next(c: *mut gsl_multiset) -> enums::Value;
    pub fn gsl_multiset_prev(c: *mut gsl_multiset) -> enums::Value;
}

#[repr(C)]
pub struct gsl_multiset {
    pub n: size_t,
    pub k: size_t,
    pub data: *mut size_t,
}
