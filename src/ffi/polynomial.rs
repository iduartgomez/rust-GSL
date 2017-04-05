//! Polynomials

use libc::{c_double, c_int, size_t};

use super::{gsl_complex, gsl_complex_packed_ptr};
use enums;

extern "C" {
    // Polynomial Evaluation
    pub fn gsl_poly_eval(c: *const c_double, len: c_int, x: c_double) -> c_double;
    pub fn gsl_poly_complex_eval(c: *const c_double, len: c_int, z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_poly_complex_eval(c: *const gsl_complex,
                                         len: c_int,
                                         z: gsl_complex)
                                         -> gsl_complex;
    pub fn gsl_poly_eval_derivs(c: *const c_double,
                                lenc: size_t,
                                x: c_double,
                                res: *mut c_double,
                                lenres: size_t)
                                -> enums::Value;
    // Divided Difference Representation of Polynomials
    pub fn gsl_poly_dd_init(dd: *mut c_double,
                            xa: *const c_double,
                            ya: *const c_double,
                            size: size_t)
                            -> enums::Value;
    pub fn gsl_poly_dd_eval(dd: *const c_double,
                            xa: *const c_double,
                            size: size_t,
                            x: c_double)
                            -> c_double;
    pub fn gsl_poly_dd_taylor(c: *mut c_double,
                              xp: c_double,
                              dd: *const c_double,
                              xa: *const c_double,
                              size: size_t,
                              w: *mut c_double)
                              -> enums::Value;
    pub fn gsl_poly_dd_hermite_init(dd: *mut c_double,
                                    za: *mut c_double,
                                    xa: *const c_double,
                                    ya: *const c_double,
                                    dya: *const c_double,
                                    size: size_t)
                                    -> enums::Value;
    // Quadratic Equations
    pub fn gsl_poly_solve_quadratic(a: c_double,
                                    b: c_double,
                                    c: c_double,
                                    x0: *mut c_double,
                                    x1: *mut c_double)
                                    -> c_int;
    pub fn gsl_poly_complex_solve_quadratic(a: c_double,
                                            b: c_double,
                                            c: c_double,
                                            x0: *mut gsl_complex,
                                            x1: *mut gsl_complex)
                                            -> c_int;
    // Cubic Equations
    pub fn gsl_poly_solve_cubic(a: c_double,
                                b: c_double,
                                c: c_double,
                                x0: *mut c_double,
                                x1: *mut c_double,
                                x2: *mut c_double)
                                -> c_int;
    pub fn gsl_poly_complex_solve_cubic(a: c_double,
                                        b: c_double,
                                        c: c_double,
                                        x0: *mut gsl_complex,
                                        x1: *mut gsl_complex,
                                        x2: *mut gsl_complex)
                                        -> c_int;
    // General Polynomial Equations
    pub fn gsl_poly_complex_workspace_alloc(n: size_t) -> *mut gsl_poly_complex_workspace;
    pub fn gsl_poly_complex_workspace_free(w: *mut gsl_poly_complex_workspace);
    pub fn gsl_poly_complex_solve(a: *const c_double,
                                  n: size_t,
                                  w: *mut gsl_poly_complex_workspace,
                                  z: gsl_complex_packed_ptr)
                                  -> enums::Value;
}

#[repr(C)]
pub struct gsl_poly_complex_workspace {
    pub nc: size_t,
    pub matrix: *mut c_double,
}
