//! Permutations

use libc::{c_double, size_t};

use super::gsl_vector;
use enums;

extern "C" {
    pub fn gsl_permutation_alloc(size: size_t) -> *mut gsl_permutation;
    pub fn gsl_permutation_calloc(size: size_t) -> *mut gsl_permutation;
    pub fn gsl_permutation_init(p: *mut gsl_permutation);
    pub fn gsl_permutation_free(p: *mut gsl_permutation);
    pub fn gsl_permutation_memcpy(dest: *mut gsl_permutation,
                                  src: *const gsl_permutation)
                                  -> enums::Value;
    pub fn gsl_permutation_get(p: *const gsl_permutation, i: size_t) -> size_t;
    pub fn gsl_permutation_swap(p: *mut gsl_permutation, i: size_t, j: size_t) -> enums::Value;
    pub fn gsl_permutation_size(p: *const gsl_permutation) -> size_t;
    //pub fn gsl_permutation_data(p: *const gsl_permutation) -> *mut size_t;
    pub fn gsl_permutation_valid(p: *const gsl_permutation) -> enums::Value;
    pub fn gsl_permutation_reverse(p: *mut gsl_permutation);
    pub fn gsl_permutation_inverse(inv: *mut gsl_permutation,
                                   p: *const gsl_permutation)
                                   -> enums::Value;
    pub fn gsl_permutation_next(p: *mut gsl_permutation) -> enums::Value;
    pub fn gsl_permutation_prev(p: *mut gsl_permutation) -> enums::Value;
    pub fn gsl_permute(p: *const size_t,
                       data: *mut c_double,
                       stride: size_t,
                       n: size_t)
                       -> enums::Value;
    pub fn gsl_permute_inverse(p: *const size_t,
                               data: *mut c_double,
                               stride: size_t,
                               n: size_t)
                               -> enums::Value;
    pub fn gsl_permute_vector(p: *const gsl_permutation, v: *mut gsl_vector) -> enums::Value;
    pub fn gsl_permute_vector_inverse(p: *const gsl_permutation,
                                      v: *mut gsl_vector)
                                      -> enums::Value;
    pub fn gsl_permutation_mul(p: *mut gsl_permutation,
                               pa: *const gsl_permutation,
                               pb: *const gsl_permutation)
                               -> enums::Value;
    pub fn gsl_permutation_linear_to_canonical(q: *mut gsl_permutation,
                                               p: *const gsl_permutation)
                                               -> enums::Value;
    pub fn gsl_permutation_canonical_to_linear(p: *mut gsl_permutation,
                                               q: *const gsl_permutation)
                                               -> enums::Value;
    pub fn gsl_permutation_inversions(p: *const gsl_permutation) -> size_t;
    pub fn gsl_permutation_linear_cycles(p: *const gsl_permutation) -> size_t;
    pub fn gsl_permutation_canonical_cycles(p: *const gsl_permutation) -> size_t;
}

#[repr(C)]
pub struct gsl_permutation {
    pub size: size_t,
    pub data: *mut size_t,
}
