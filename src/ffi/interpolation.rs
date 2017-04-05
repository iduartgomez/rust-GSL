//! Interpolation Functions

use libc::{c_double, c_uint, c_void, size_t, c_char};

use enums;

extern "C" {
    pub static gsl_interp_linear: *const gsl_interp_type;
    pub static gsl_interp_polynomial: *const gsl_interp_type;
    pub static gsl_interp_cspline: *const gsl_interp_type;
    pub static gsl_interp_cspline_periodic: *const gsl_interp_type;
    pub static gsl_interp_akima: *const gsl_interp_type;
    pub static gsl_interp_akima_periodic: *const gsl_interp_type;

    pub fn gsl_interp_alloc(t: *const gsl_interp_type, size: size_t) -> *mut gsl_interp;
    pub fn gsl_interp_init(interp: *mut gsl_interp,
                           xa: *const c_double,
                           ya: *const c_double,
                           size: size_t)
                           -> enums::Value;
    pub fn gsl_interp_free(interp: *mut gsl_interp);
    pub fn gsl_interp_min_size(interp: *const gsl_interp) -> c_uint;
    pub fn gsl_interp_name(interp: *const gsl_interp) -> *const c_char;
    // Interpolation Types
    pub fn gsl_interp_type_min_size(t: *const gsl_interp_type) -> c_uint;
    // Index Look-up and Acceleration
    pub fn gsl_interp_accel_find(a: *mut ::InterpAccel,
                                 x_array: *const c_double,
                                 size: size_t,
                                 x: c_double)
                                 -> size_t;
    pub fn gsl_interp_bsearch(x_array: *const c_double,
                              x: c_double,
                              index_lo: size_t,
                              index_hi: size_t)
                              -> size_t;
    // Evaluation of Interpolating Functions
    pub fn gsl_interp_eval(interp: *const gsl_interp,
                           xa: *const c_double,
                           ya: *const c_double,
                           x: c_double,
                           acc: *mut ::InterpAccel)
                           -> c_double;
    pub fn gsl_interp_eval_e(interp: *const gsl_interp,
                             xa: *const c_double,
                             ya: *const c_double,
                             x: c_double,
                             acc: *mut ::InterpAccel,
                             y: *mut c_double)
                             -> enums::Value;
    pub fn gsl_interp_eval_deriv(interp: *const gsl_interp,
                                 xa: *const c_double,
                                 ya: *const c_double,
                                 x: c_double,
                                 acc: *mut ::InterpAccel)
                                 -> c_double;
    pub fn gsl_interp_eval_deriv_e(interp: *const gsl_interp,
                                   xa: *const c_double,
                                   ya: *const c_double,
                                   x: c_double,
                                   acc: *mut ::InterpAccel,
                                   d: *mut c_double)
                                   -> enums::Value;
    pub fn gsl_interp_eval_deriv2(interp: *const gsl_interp,
                                  xa: *const c_double,
                                  ya: *const c_double,
                                  x: c_double,
                                  acc: *mut ::InterpAccel)
                                  -> c_double;
    pub fn gsl_interp_eval_deriv2_e(interp: *const gsl_interp,
                                    xa: *const c_double,
                                    ya: *const c_double,
                                    x: c_double,
                                    acc: *mut ::InterpAccel,
                                    d2: *mut c_double)
                                    -> enums::Value;
    pub fn gsl_interp_eval_integ(interp: *const gsl_interp,
                                 xa: *const c_double,
                                 ya: *const c_double,
                                 a: c_double,
                                 b: c_double,
                                 acc: *mut ::InterpAccel)
                                 -> c_double;
    pub fn gsl_interp_eval_integ_e(interp: *const gsl_interp,
                                   xa: *const c_double,
                                   ya: *const c_double,
                                   a: c_double,
                                   b: c_double,
                                   acc: *mut ::InterpAccel,
                                   result: *mut c_double)
                                   -> enums::Value;
    // Higher-level Interface
    pub fn gsl_spline_alloc(t: *const gsl_interp_type, size: size_t) -> *mut gsl_spline;
    pub fn gsl_spline_init(spline: *mut gsl_spline,
                           xa: *const c_double,
                           ya: *const c_double,
                           size: size_t)
                           -> enums::Value;
    pub fn gsl_spline_free(spline: *mut gsl_spline);
    pub fn gsl_spline_min_size(spline: *const gsl_spline) -> c_uint;
    pub fn gsl_spline_name(spline: *const gsl_spline) -> *const c_char;
    pub fn gsl_spline_eval(spline: *const gsl_spline,
                           x: c_double,
                           acc: *mut ::InterpAccel)
                           -> c_double;
    pub fn gsl_spline_eval_e(spline: *const gsl_spline,
                             x: c_double,
                             acc: *mut ::InterpAccel,
                             y: *mut c_double)
                             -> enums::Value;
    pub fn gsl_spline_eval_deriv(spline: *const gsl_spline,
                                 x: c_double,
                                 acc: *mut ::InterpAccel)
                                 -> c_double;
    pub fn gsl_spline_eval_deriv_e(spline: *const gsl_spline,
                                   x: c_double,
                                   acc: *mut ::InterpAccel,
                                   d: *mut c_double)
                                   -> enums::Value;
    pub fn gsl_spline_eval_deriv2(spline: *const gsl_spline,
                                  x: c_double,
                                  acc: *mut ::InterpAccel)
                                  -> c_double;
    pub fn gsl_spline_eval_deriv2_e(spline: *const gsl_spline,
                                    x: c_double,
                                    acc: *mut ::InterpAccel,
                                    d2: *mut c_double)
                                    -> enums::Value;
    pub fn gsl_spline_eval_integ(spline: *const gsl_spline,
                                 a: c_double,
                                 b: c_double,
                                 acc: *mut ::InterpAccel)
                                 -> c_double;
    pub fn gsl_spline_eval_integ_e(spline: *const gsl_spline,
                                   a: c_double,
                                   b: c_double,
                                   acc: *mut ::InterpAccel,
                                   result: *mut c_double)
                                   -> enums::Value;
    pub fn gsl_min_test_interval(x_lower: c_double,
                                 x_upper: c_double,
                                 epsabs: c_double,
                                 epsrel: c_double)
                                 -> enums::Value;
}

/* interpolation object type */
#[repr(C)]
pub struct gsl_interp_type {
    pub name: *const c_char,
    pub min_size: c_uint,
    pub alloc: Option<extern "C" fn(size_t) -> *mut c_void>,
    pub init: Option<extern "C" fn(*mut c_void, *const c_double, *const c_double, size_t)
                                   -> enums::Value>,
    pub eval: Option<extern "C" fn(*const c_void,
                                   *const c_double,
                                   *const c_double,
                                   size_t,
                                   c_double,
                                   *mut ::InterpAccel,
                                   *mut c_double)
                                   -> enums::Value>,
    pub eval_deriv: Option<extern "C" fn(*const c_void,
                                         *const c_double,
                                         *const c_double,
                                         size_t,
                                         c_double,
                                         *mut ::InterpAccel,
                                         *mut c_double)
                                         -> enums::Value>,
    pub eval_deriv2: Option<extern "C" fn(*const c_void,
                                          *const c_double,
                                          *const c_double,
                                          size_t,
                                          c_double,
                                          *mut ::InterpAccel,
                                          *mut c_double)
                                          -> enums::Value>,
    pub eval_integ: Option<extern "C" fn(*const c_void,
                                         *const c_double,
                                         *const c_double,
                                         size_t,
                                         c_double,
                                         *mut ::InterpAccel,
                                         c_double,
                                         c_double,
                                         *mut c_double)
                                         -> enums::Value>,
    pub free: Option<extern "C" fn(*mut c_void)>,
}

/* general interpolation object */
#[repr(C)]
pub struct gsl_interp {
    pub _type: *const gsl_interp_type,
    pub xmin: c_double,
    pub xmax: c_double,
    pub size: size_t,
    pub state: *mut c_void,
}

/* general interpolation object */
#[repr(C)]
pub struct gsl_spline {
    pub interp: *mut gsl_interp,
    pub x: *mut c_double,
    pub y: *mut c_double,
    pub size: size_t,
}
