//! Sorting functions

use libc::{c_double, size_t};

use super::{gsl_vector, gsl_permutation};
use enums;

extern "C" {
    // Sorting objects
    //pub fn gsl_heapsort(array: *mut c_void, count: size_t, size: size_t, compare: compare_fn);
    //pub fn gsl_heapsort_index(p: *mut size_t, array: *const c_void, count: size_t, size: size_t, compare: compare_fn) -> enums::Value;
    // Sorting vectors
    pub fn gsl_sort(data: *mut c_double, stride: size_t, n: size_t);
    pub fn gsl_sort2(data1: *mut c_double,
                     stride1: size_t,
                     data2: *mut c_double,
                     stride2: size_t,
                     n: size_t);
    pub fn gsl_sort_vector(v: *mut gsl_vector);
    pub fn gsl_sort_vector2(v1: *mut gsl_vector, v2: *mut gsl_vector);
    pub fn gsl_sort_index(p: *mut size_t, data: *const c_double, stride: size_t, n: size_t);
    pub fn gsl_sort_vector_index(p: *mut gsl_permutation, v: *const gsl_vector) -> enums::Value;
    // Selecting the k smallest or largest elements
    pub fn gsl_sort_smallest(dest: *mut c_double,
                             k: size_t,
                             src: *const c_double,
                             stride: size_t,
                             n: size_t)
                             -> enums::Value;
    pub fn gsl_sort_largest(dest: *mut c_double,
                            k: size_t,
                            src: *const c_double,
                            stride: size_t,
                            n: size_t)
                            -> enums::Value;
    pub fn gsl_sort_vector_smallest(dest: *mut c_double,
                                    k: size_t,
                                    v: *const gsl_vector)
                                    -> enums::Value;
    pub fn gsl_sort_vector_largest(dest: *mut c_double,
                                   k: size_t,
                                   v: *const gsl_vector)
                                   -> enums::Value;
    pub fn gsl_sort_smallest_index(p: *mut size_t,
                                   k: size_t,
                                   src: *const c_double,
                                   stride: size_t,
                                   n: size_t)
                                   -> enums::Value;
    pub fn gsl_sort_largest_index(p: *mut size_t,
                                  k: size_t,
                                  src: *const c_double,
                                  stride: size_t,
                                  n: size_t)
                                  -> enums::Value;
    pub fn gsl_sort_vector_smallest_index(p: *mut size_t,
                                          k: size_t,
                                          v: *const gsl_vector)
                                          -> enums::Value;
    pub fn gsl_sort_vector_largest_index(p: *mut size_t,
                                         k: size_t,
                                         v: *const gsl_vector)
                                         -> enums::Value;
}
