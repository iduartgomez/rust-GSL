//! Fitting functions

use libc::{c_double, size_t};

use enums;

extern "C" {
    pub fn gsl_fit_linear(x: *const c_double,
                          xstride: size_t,
                          y: *const c_double,
                          ystride: size_t,
                          n: size_t,
                          c0: *mut c_double,
                          c1: *mut c_double,
                          cov00: *mut c_double,
                          cov01: *mut c_double,
                          cov11: *mut c_double,
                          sumsq: c_double)
                          -> enums::Value;
    pub fn gsl_fit_wlinear(x: *const c_double,
                           xstride: size_t,
                           w: *const c_double,
                           wstride: size_t,
                           y: *const c_double,
                           ystride: size_t,
                           n: size_t,
                           c0: *mut c_double,
                           c1: *mut c_double,
                           cov00: *mut c_double,
                           cov01: *mut c_double,
                           cov11: *mut c_double,
                           chisq: *mut c_double)
                           -> enums::Value;
    pub fn gsl_fit_linear_est(x: c_double,
                              c0: c_double,
                              c1: c_double,
                              cov00: c_double,
                              cov01: c_double,
                              cov11: c_double,
                              y: *mut c_double,
                              y_err: *mut c_double)
                              -> enums::Value;
    pub fn gsl_fit_mul(x: *const c_double,
                       xstride: size_t,
                       y: *const c_double,
                       ystride: size_t,
                       n: size_t,
                       c1: *mut c_double,
                       cov11: *mut c_double,
                       sumsq: *mut c_double)
                       -> enums::Value;
    pub fn gsl_fit_wmul(x: *const c_double,
                        xstride: size_t,
                        w: *const c_double,
                        wstride: size_t,
                        y: *const c_double,
                        ystride: size_t,
                        n: size_t,
                        c1: *mut c_double,
                        cov11: *mut c_double,
                        sumsq: *mut c_double)
                        -> enums::Value;
    pub fn gsl_fit_mul_est(x: c_double,
                           c1: c_double,
                           cov11: c_double,
                           y: *mut c_double,
                           y_err: *mut c_double)
                           -> enums::Value;
}
