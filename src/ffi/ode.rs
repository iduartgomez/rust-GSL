//! Ordinary Differential Equations

use libc::{c_double, c_int, c_uint, size_t, c_char, c_void};

use enums;

extern "C" {
    pub static gsl_odeiv2_step_rk2: *const gsl_odeiv2_step_type;
    pub static gsl_odeiv2_step_rk4: *const gsl_odeiv2_step_type;
    pub static gsl_odeiv2_step_rkf45: *const gsl_odeiv2_step_type;
    pub static gsl_odeiv2_step_rkck: *const gsl_odeiv2_step_type;
    pub static gsl_odeiv2_step_rk8pd: *const gsl_odeiv2_step_type;
    pub static gsl_odeiv2_step_rk1imp: *const gsl_odeiv2_step_type;
    pub static gsl_odeiv2_step_rk2imp: *const gsl_odeiv2_step_type;
    pub static gsl_odeiv2_step_rk4imp: *const gsl_odeiv2_step_type;
    pub static gsl_odeiv2_step_bsimp: *const gsl_odeiv2_step_type;
    pub static gsl_odeiv2_step_msadams: *const gsl_odeiv2_step_type;
    pub static gsl_odeiv2_step_msbdf: *const gsl_odeiv2_step_type;

    pub static gsl_odeiv2_control_scaled: *const gsl_odeiv2_control_type;
    pub static gsl_odeiv2_control_standard: *const gsl_odeiv2_control_type;

    // Stepping Functions
    pub fn gsl_odeiv2_step_alloc(t: *const gsl_odeiv2_step_type,
                                 dim: size_t)
                                 -> *mut gsl_odeiv2_step;
    pub fn gsl_odeiv2_step_reset(s: *mut gsl_odeiv2_step) -> enums::Value;
    pub fn gsl_odeiv2_step_free(s: *mut gsl_odeiv2_step);
    pub fn gsl_odeiv2_step_name(s: *mut gsl_odeiv2_step) -> *const c_char;
    pub fn gsl_odeiv2_step_order(s: *const gsl_odeiv2_step) -> c_uint;
    pub fn gsl_odeiv2_step_set_driver(s: *mut gsl_odeiv2_step,
                                      d: *const gsl_odeiv2_driver)
                                      -> enums::Value;
    pub fn gsl_odeiv2_step_apply(s: *mut gsl_odeiv2_step,
                                 t: c_double,
                                 h: c_double,
                                 y: *mut c_double,
                                 yerr: *mut c_double,
                                 dydt_in: *const c_double,
                                 dydt_out: *mut c_double,
                                 sys: *const gsl_odeiv2_system)
                                 -> enums::Value;
    // Adaptive Step-size Control
    pub fn gsl_odeiv2_control_standard_new(eps_abs: c_double,
                                           eps_rel: c_double,
                                           a_y: c_double,
                                           a_dydt: c_double)
                                           -> *mut gsl_odeiv2_control;
    pub fn gsl_odeiv2_control_y_new(eps_abs: c_double,
                                    eps_rel: c_double)
                                    -> *mut gsl_odeiv2_control;
    pub fn gsl_odeiv2_control_yp_new(eps_abs: c_double,
                                     eps_rel: c_double)
                                     -> *mut gsl_odeiv2_control;
    pub fn gsl_odeiv2_control_scaled_new(eps_abs: c_double,
                                         eps_rel: c_double,
                                         a_y: c_double,
                                         a_dydt: c_double,
                                         scale_abs: *const c_double,
                                         dim: size_t)
                                         -> *mut gsl_odeiv2_control;
    pub fn gsl_odeiv2_control_alloc(t: *const gsl_odeiv2_control_type) -> *mut gsl_odeiv2_control;
    pub fn gsl_odeiv2_control_init(c: *mut gsl_odeiv2_control,
                                   eps_abs: c_double,
                                   eps_rel: c_double,
                                   a_y: c_double,
                                   a_dydt: c_double)
                                   -> enums::Value;
    pub fn gsl_odeiv2_control_free(c: *mut gsl_odeiv2_control);
    pub fn gsl_odeiv2_control_hadjust(c: *mut gsl_odeiv2_control,
                                      s: *mut gsl_odeiv2_step,
                                      y: *const c_double,
                                      yerr: *const c_double,
                                      dydt: *const c_double,
                                      h: *mut c_double)
                                      -> ::ODEiv;
    pub fn gsl_odeiv2_control_name(c: *const gsl_odeiv2_control) -> *const c_char;
    pub fn gsl_odeiv2_control_errlevel(c: *mut gsl_odeiv2_control,
                                       y: c_double,
                                       dydt: c_double,
                                       h: c_double,
                                       ind: size_t,
                                       errlev: *mut c_double)
                                       -> enums::Value;
    pub fn gsl_odeiv2_control_set_driver(c: *mut gsl_odeiv2_control,
                                         d: *const gsl_odeiv2_driver)
                                         -> enums::Value;
    // Evolution
    pub fn gsl_odeiv2_evolve_alloc(dim: size_t) -> *mut gsl_odeiv2_evolve;
    pub fn gsl_odeiv2_evolve_apply(e: *mut gsl_odeiv2_evolve,
                                   con: *mut gsl_odeiv2_control,
                                   step: *mut gsl_odeiv2_step,
                                   sys: *const gsl_odeiv2_system,
                                   t: *mut c_double,
                                   t1: c_double,
                                   h: *mut c_double,
                                   y: *mut c_double)
                                   -> enums::Value;
    pub fn gsl_odeiv2_evolve_apply_fixed_step(e: *mut gsl_odeiv2_evolve,
                                              con: *mut gsl_odeiv2_control,
                                              step: *mut gsl_odeiv2_step,
                                              sys: *const gsl_odeiv2_system,
                                              t: *mut c_double,
                                              h: c_double,
                                              y: *mut c_double)
                                              -> enums::Value;
    pub fn gsl_odeiv2_evolve_reset(e: *mut gsl_odeiv2_evolve) -> enums::Value;
    pub fn gsl_odeiv2_evolve_free(e: *mut gsl_odeiv2_evolve);
    pub fn gsl_odeiv2_evolve_set_driver(e: *mut gsl_odeiv2_evolve,
                                        d: *const gsl_odeiv2_driver)
                                        -> enums::Value;
    // Driver
    pub fn gsl_odeiv2_driver_alloc_y_new(sys: *const gsl_odeiv2_system,
                                         t: *const gsl_odeiv2_step_type,
                                         hstart: c_double,
                                         epsabs: c_double,
                                         epsrel: c_double)
                                         -> *mut gsl_odeiv2_driver;
    pub fn gsl_odeiv2_driver_alloc_yp_new(sys: *const gsl_odeiv2_system,
                                          t: *const gsl_odeiv2_step_type,
                                          hstart: c_double,
                                          epsabs: c_double,
                                          epsrel: c_double)
                                          -> *mut gsl_odeiv2_driver;
    pub fn gsl_odeiv2_driver_alloc_standard_new(sys: *const gsl_odeiv2_system,
                                                t: *const gsl_odeiv2_step_type,
                                                hstart: c_double,
                                                epsabs: c_double,
                                                epsrel: c_double,
                                                a_y: c_double,
                                                a_dydt: c_double)
                                                -> *mut gsl_odeiv2_driver;
    pub fn gsl_odeiv2_driver_alloc_scaled_new(sys: *const gsl_odeiv2_system,
                                              t: *const gsl_odeiv2_step_type,
                                              hstart: c_double,
                                              epsabs: c_double,
                                              epsrel: c_double,
                                              a_y: c_double,
                                              a_dydt: c_double,
                                              scale_abs: *const c_double)
                                              -> *mut gsl_odeiv2_driver;
    pub fn gsl_odeiv2_driver_set_hmin(d: *mut gsl_odeiv2_driver, hmin: c_double) -> enums::Value;
    pub fn gsl_odeiv2_driver_set_hmax(d: *mut gsl_odeiv2_driver, hmax: c_double) -> enums::Value;
    pub fn gsl_odeiv2_driver_set_nmax(d: *mut gsl_odeiv2_driver, nmax: usize) -> enums::Value;
    pub fn gsl_odeiv2_driver_apply(d: *mut gsl_odeiv2_driver,
                                   t: *mut c_double,
                                   t1: c_double,
                                   y: *mut c_double)
                                   -> enums::Value;
    pub fn gsl_odeiv2_driver_apply_fixed_step(d: *mut gsl_odeiv2_driver,
                                              t: *mut c_double,
                                              h: c_double,
                                              n: usize,
                                              y: *mut c_double)
                                              -> enums::Value;
    pub fn gsl_odeiv2_driver_reset(d: *mut gsl_odeiv2_driver) -> enums::Value;
    pub fn gsl_odeiv2_driver_reset_hstart(d: *mut gsl_odeiv2_driver,
                                          hstart: c_double)
                                          -> enums::Value;
    pub fn gsl_odeiv2_driver_free(d: *mut gsl_odeiv2_driver);
}


#[repr(C)]
pub struct gsl_odeiv2_system {
    pub function: extern "C" fn(t: c_double, *const c_double, *mut c_double, *mut c_void)
                                -> enums::Value,
    pub jacobian: Option<extern "C" fn(t: c_double,
                                       *const c_double,
                                       *mut c_double,
                                       *mut c_double,
                                       *mut c_void)
                                       -> enums::Value>,
    pub dimension: usize,
    pub params: *mut c_void,
}

#[repr(C)]
pub struct gsl_odeiv2_driver {
    // ODE system
    pub sys: *const gsl_odeiv2_system,
    // stepper object
    pub s: *mut gsl_odeiv2_step,
    // control object
    pub c: *mut gsl_odeiv2_control,
    // evolve object
    pub e: *mut gsl_odeiv2_evolve,
    // step size
    pub h: c_double,
    // minimum step size allowed
    pub hmin: c_double,
    // maximum step size allowed
    pub hmax: c_double,
    // number of steps taken
    pub n: usize,
    // Maximum number of steps allowed
    pub nmax: usize,
}

#[repr(C)]
pub struct gsl_odeiv2_evolve {
    pub dimension: size_t,
    pub y0: *mut c_double,
    pub yerr: *mut c_double,
    pub dydt_in: *mut c_double,
    pub dydt_out: *mut c_double,
    pub last_step: c_double,
    pub count: usize,
    pub failed_steps: usize,
    pub driver: *const gsl_odeiv2_driver,
}

#[repr(C)]
pub struct gsl_odeiv2_control {
    pub type_: *const gsl_odeiv2_control_type,
    pub state: *mut c_void,
}

#[repr(C)]
pub struct gsl_odeiv2_control_type {
    pub name: *const c_char,
    pub alloc: fn() -> *mut c_void,
    pub init: fn(state: *mut c_void,
                 eps_abs: c_double,
                 eps_rel: c_double,
                 a_y: c_double,
                 a_dydt: c_double)
                 -> enums::Value,
    pub hadjust: fn(state: *mut c_void,
                    dim: size_t,
                    ord: c_uint,
                    y: *const c_double,
                    yerr: *const c_double,
                    yp: *const c_double,
                    h: *mut c_double)
                    -> enums::Value,
    pub errlevel: fn(state: *mut c_void,
                     y: c_double,
                     dydt: c_double,
                     h: c_double,
                     ind: size_t,
                     errlev: *mut c_double)
                     -> enums::Value,
    pub set_driver: fn(state: *mut c_void, d: *const gsl_odeiv2_driver) -> enums::Value,
    pub free: fn(state: *mut c_void),
}

#[repr(C)]
pub struct gsl_odeiv2_step {
    pub type_: *const gsl_odeiv2_step_type,
    pub dimension: size_t,
    pub state: *mut c_void,
}

#[repr(C)]
pub struct gsl_odeiv2_step_type {
    pub name: *const c_char,
    pub can_use_dydt_in: c_int,
    pub gives_exact_dydt_out: c_int,
    pub alloc: fn(dim: size_t) -> *mut c_void,
    pub apply: fn(state: *mut c_void,
                  dim: size_t,
                  t: c_double,
                  h: c_double,
                  y: *mut c_double,
                  yerr: *mut c_double,
                  dydt_in: *const c_double,
                  dydt_out: *mut c_double,
                  dydt: *const gsl_odeiv2_system)
                  -> enums::Value,
    pub set_driver: fn(state: *mut c_void, d: *const gsl_odeiv2_driver) -> enums::Value,
    pub reset: fn(state: *mut c_void, dim: size_t) -> enums::Value,
    pub order: fn(state: *mut c_void) -> c_uint,
    pub free: fn(state: *mut c_void),
}
