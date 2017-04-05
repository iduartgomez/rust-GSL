//! Integration

use libc::{c_double, c_int, size_t};

use enums;

extern "C" {
    // QAG adaptive integration
    pub fn gsl_integration_workspace_alloc(n: size_t) -> *mut gsl_integration_workspace;
    pub fn gsl_integration_workspace_free(w: *mut gsl_integration_workspace);
    // QAWS adaptive integration for singular functions
    pub fn gsl_integration_qaws_table_alloc(alpha: c_double,
                                            beta: c_double,
                                            mu: c_int,
                                            nu: c_int)
                                            -> *mut gsl_integration_qaws_table;
    pub fn gsl_integration_qaws_table_set(t: *mut gsl_integration_qaws_table,
                                          alpha: c_double,
                                          beta: c_double,
                                          mu: c_int,
                                          nu: c_int)
                                          -> enums::Value;
    pub fn gsl_integration_qaws_table_free(t: *mut gsl_integration_qaws_table);
    // QAWO adaptive integration for oscillatory functions
    pub fn gsl_integration_qawo_table_alloc(omega: c_double,
                                            l: c_double,
                                            sine: ::IntegrationQawo,
                                            n: size_t)
                                            -> *mut gsl_integration_qawo_table;
    pub fn gsl_integration_qawo_table_set(t: *mut gsl_integration_qawo_table,
                                          omega: c_double,
                                          l: c_double,
                                          sine: ::IntegrationQawo)
                                          -> enums::Value;
    pub fn gsl_integration_qawo_table_set_length(t: *mut gsl_integration_qawo_table,
                                                 l: c_double)
                                                 -> enums::Value;
    pub fn gsl_integration_qawo_table_free(t: *mut gsl_integration_qawo_table);
    // CQUAD doubly-adaptive integration
    pub fn gsl_integration_cquad_workspace_alloc(n: size_t)
                                                 -> *mut gsl_integration_cquad_workspace;
    pub fn gsl_integration_cquad_workspace_free(w: *mut gsl_integration_cquad_workspace);
    // Gauss-Legendre integration
    pub fn gsl_integration_glfixed_table_alloc(n: size_t) -> *mut gsl_integration_glfixed_table;
    pub fn gsl_integration_glfixed_point(a: c_double,
                                         b: c_double,
                                         i: size_t,
                                         xi: *mut c_double,
                                         wi: *mut c_double,
                                         t: *const gsl_integration_glfixed_table)
                                         -> enums::Value;
    pub fn gsl_integration_glfixed_table_free(t: *mut gsl_integration_glfixed_table);

}

#[repr(C)]
pub struct gsl_integration_workspace {
    pub limit: size_t,
    pub size: size_t,
    pub nrmax: size_t,
    pub i: size_t,
    pub maximum_level: size_t,
    pub alist: *mut c_double,
    pub blist: *mut c_double,
    pub rlist: *mut c_double,
    pub elist: *mut c_double,
    pub order: *mut size_t,
    pub level: *mut size_t,
}

#[repr(C)]
pub struct extrapolation_table {
    pub n: size_t,
    pub rlist2: [c_double; 52],
    pub nres: size_t,
    pub res3la: [c_double; 3],
}

#[repr(C)]
pub struct gsl_integration_qaws_table {
    pub alpha: c_double,
    pub beta: c_double,
    pub mu: c_int,
    pub nu: c_int,
    pub ri: [c_double; 25],
    pub rj: [c_double; 25],
    pub rg: [c_double; 25],
    pub rh: [c_double; 25],
}

#[repr(C)]
pub struct gsl_integration_qawo_table {
    pub n: size_t,
    pub omega: c_double,
    pub L: c_double,
    pub par: c_double,
    pub sine: ::IntegrationQawo,
    pub chebmo: *mut c_double,
}

/* Data of a single interval */
#[repr(C)]
pub struct gsl_integration_cquad_ival {
    pub a: c_double,
    pub b: c_double,
    pub c: [c_double; 64],
    pub fx: [c_double; 33],
    pub igral: c_double,
    pub err: c_double,
    pub depth: c_int,
    pub rdepth: c_int,
    pub ndiv: c_int,
}


/* The workspace is just a collection of intervals */
#[repr(C)]
pub struct gsl_integration_cquad_workspace {
    pub size: size_t,
    pub ivals: *mut gsl_integration_cquad_ival,
    pub heap: *mut size_t,
}

/* Workspace for fixed-order Gauss-Legendre integration */
#[repr(C)]
pub struct gsl_integration_glfixed_table {
    pub n: size_t, /* number of points */
    pub x: *mut c_double, /* Gauss abscissae/points */
    pub w: *mut c_double, /* Gauss weights for each abscissae */
    pub precomputed: c_int, /* high precision abscissae/weights precomputed? */
}
