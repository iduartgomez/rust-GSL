//! Discrete Hankel functions

use libc::{c_double, c_int, size_t};

use enums;

extern "C" {
    pub fn gsl_dht_alloc(size: size_t) -> *mut gsl_dht;
    pub fn gsl_dht_init(t: *mut gsl_dht, nu: c_double, xmax: c_double) -> enums::Value;
    pub fn gsl_dht_new(size: size_t, nu: c_double, xmax: c_double) -> *mut gsl_dht;
    pub fn gsl_dht_free(t: *mut gsl_dht);
    pub fn gsl_dht_apply(t: *const gsl_dht,
                         f_in: *const c_double,
                         f_out: *mut c_double)
                         -> enums::Value;
    pub fn gsl_dht_x_sample(t: *const gsl_dht, n: c_int) -> c_double;
    pub fn gsl_dht_k_sample(t: *const gsl_dht, n: c_int) -> c_double;
}

#[repr(C)]
pub struct gsl_dht {
    pub size: size_t, // size of the sample arrays to be transformed
    pub nu: c_double, // Bessel function order
    pub xmax: c_double, // the upper limit to the x-sampling domain
    pub kmax: c_double, // the upper limit to the k-sampling domain
    pub j: *mut c_double, // array of computed J_nu zeros, j_{nu,s} = j[s]
    pub Jjj: *mut c_double, // transform numerator, J_nu(j_i j_m / j_N)
    pub J2: *mut c_double, // transform denominator, J_{nu+1}^2(j_m)
}
