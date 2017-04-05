//! Basis Splines

use libc::{c_double, size_t};

use super::gsl_vector;
use enums;

extern "C" {
    pub fn gsl_bspline_alloc(k: size_t, nbreak: size_t) -> *mut gsl_bspline_workspace;
    pub fn gsl_bspline_free(w: *mut gsl_bspline_workspace);
    #[cfg(not(feature = "v2"))]
    pub fn gsl_bspline_deriv_alloc(k: size_t) -> *mut gsl_bspline_deriv_workspace;
    #[cfg(not(feature = "v2"))]
    pub fn gsl_bspline_deriv_free(w: *mut gsl_bspline_deriv_workspace);
    pub fn gsl_bspline_knots(breakpts: *const gsl_vector,
                             w: *mut gsl_bspline_workspace)
                             -> enums::Value;
    pub fn gsl_bspline_knots_uniform(a: c_double,
                                     b: c_double,
                                     w: *mut gsl_bspline_workspace)
                                     -> enums::Value;
    pub fn gsl_bspline_eval(x: c_double,
                            B: *mut gsl_vector,
                            w: *mut gsl_bspline_workspace)
                            -> enums::Value;
    pub fn gsl_bspline_eval_nonzero(x: c_double,
                                    Bk: *mut gsl_vector,
                                    istart: *mut size_t,
                                    iend: *mut size_t,
                                    w: *mut gsl_bspline_workspace)
                                    -> enums::Value;
    pub fn gsl_bspline_ncoeffs(w: *mut gsl_bspline_workspace) -> size_t;
    #[cfg(not(feature = "v2"))]
    pub fn gsl_bspline_deriv_eval(x: c_double,
                                  nderiv: size_t,
                                  dB: *mut gsl_matrix,
                                  w: *mut gsl_bspline_workspace,
                                  dw: *mut gsl_bspline_deriv_workspace)
                                  -> enums::Value;
    #[cfg(not(feature = "v2"))]
    pub fn gsl_bspline_deriv_eval_nonzero(x: c_double,
                                          nderiv: size_t,
                                          Bk: *mut gsl_matrix,
                                          istart: *mut size_t,
                                          iend: *mut size_t,
                                          w: *mut gsl_bspline_workspace,
                                          dw: *mut gsl_bspline_deriv_workspace)
                                          -> enums::Value;
    pub fn gsl_bspline_greville_abscissa(i: size_t, w: *mut gsl_bspline_workspace) -> c_double;
}


#[repr(C)]
pub struct gsl_bspline_workspace {
    pub k: size_t, // spline order
    pub km1: size_t, // k - 1 (polynomial order)
    pub l: size_t, // number of polynomial pieces on interval
    pub nbreak: size_t, // number of breakpoints (l + 1)
    pub n: size_t, // number of bspline basis functions (l + k - 1)
    pub knots: *mut gsl_vector, // knots vector
    pub deltal: *mut gsl_vector, // left delta
    pub deltar: *mut gsl_vector, // right delta
    pub B: *mut gsl_vector, // temporary spline results
}

#[repr(C)]
#[cfg(not(feature = "v2"))]
pub struct gsl_bspline_deriv_workspace {
    pub k: size_t, // spline order
    pub A: *mut gsl_matrix, // work matrix
    pub dB: *mut gsl_matrix, // temporary derivative results
}
