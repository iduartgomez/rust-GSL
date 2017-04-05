//! Chebyshev Approximations

use libc::{c_double, c_int, size_t};

use enums;

extern "C" {
    // Creation and Calculation of Chebyshev Series
    pub fn gsl_cheb_alloc(n: size_t) -> *mut gsl_cheb_series;
    pub fn gsl_cheb_free(cs: *mut gsl_cheb_series);
    // Auxiliary functions
    pub fn gsl_cheb_order(cs: *const gsl_cheb_series) -> size_t;
    pub fn gsl_cheb_size(cs: *const gsl_cheb_series) -> size_t;
    // Chebyshev Series Evaluation
    pub fn gsl_cheb_eval(cs: *const gsl_cheb_series, x: c_double) -> c_double;
    pub fn gsl_cheb_eval_err(cs: *const gsl_cheb_series,
                             x: c_double,
                             result: *mut c_double,
                             abs_err: *mut c_double)
                             -> enums::Value;
    pub fn gsl_cheb_eval_n(cs: *const gsl_cheb_series, order: size_t, x: c_double) -> c_double;
    pub fn gsl_cheb_eval_n_err(cs: *const gsl_cheb_series,
                               order: size_t,
                               x: c_double,
                               result: *mut c_double,
                               abs_err: *mut c_double)
                               -> enums::Value;
    // Derivatives and Integrals
    pub fn gsl_cheb_calc_deriv(cs: *mut gsl_cheb_series,
                               deriv: *const gsl_cheb_series)
                               -> enums::Value;
    pub fn gsl_cheb_calc_integ(cs: *mut gsl_cheb_series,
                               integ: *const gsl_cheb_series)
                               -> enums::Value;
}

#[repr(C)]
pub struct gsl_cheb_series {
    pub c: *mut c_double, // coefficients
    pub order: c_int, // order of expansion
    pub a: c_double, // lower interval point
    pub b: c_double, // upper interval point
    pub order_sp: size_t,
    pub f: *mut c_double,
}
