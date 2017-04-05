//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

#![allow(improper_ctypes)]

use libc::{c_double, c_int, c_uint, c_float, c_void, size_t, c_ulong, c_char};

use enums;

mod basis_spline;
mod blas;
mod chebyshev;
mod combination;
mod discrete_hankel;
mod fft;
mod fit;
mod histogram;
mod integration;
mod interpolation;
mod linalg;
mod monte_carlo;
mod multiset;
mod n_tuples;
mod ode;
mod permutations;
mod polynomial;
mod randist;
mod solvers;
mod sort;
mod statistics;
mod wavelet_transforms;

pub use self::basis_spline::*;
pub use self::blas::*;
pub use self::chebyshev::*;
pub use self::combination::*;
pub use self::discrete_hankel::*;
pub use self::fft::*;
pub use self::fit::*;
pub use self::histogram::*;
pub use self::integration::*;
pub use self::interpolation::*;
pub use self::linalg::*;
pub use self::monte_carlo::*;
pub use self::multiset::*;
pub use self::n_tuples::*;
pub use self::ode::*;
pub use self::permutations::*;
pub use self::polynomial::*;
pub use self::randist::*;
pub use self::solvers::*;
pub use self::sort::*;
pub use self::statistics::*;
pub use self::wavelet_transforms::*;

pub type gsl_complex_packed_ptr = *mut c_double;
pub type gsl_complex_packed_array = *mut c_double;
#[allow(dead_code)]
pub type coord = c_int;

pub trait FFI<T> {
    fn wrap(r: *mut T) -> Self;
    fn soft_wrap(r: *mut T) -> Self;
    fn unwrap_shared(&Self) -> *const T;
    fn unwrap_unique(&mut Self) -> *mut T;
}

extern "C" {
    pub static gsl_rng_mt19937: *const gsl_rng_type;
    pub static gsl_rng_ranlxs0: *const gsl_rng_type;
    pub static gsl_rng_ranlxs1: *const gsl_rng_type;
    pub static gsl_rng_ranlxs2: *const gsl_rng_type;
    pub static gsl_rng_ranlxd1: *const gsl_rng_type;
    pub static gsl_rng_ranlxd2: *const gsl_rng_type;
    pub static gsl_rng_ranlux: *const gsl_rng_type;
    pub static gsl_rng_ranlux389: *const gsl_rng_type;
    pub static gsl_rng_cmrg: *const gsl_rng_type;
    pub static gsl_rng_mrg: *const gsl_rng_type;
    pub static gsl_rng_taus: *const gsl_rng_type;
    pub static gsl_rng_taus2: *const gsl_rng_type;
    pub static gsl_rng_gfsr4: *const gsl_rng_type;

    pub static gsl_rng_rand: *const gsl_rng_type;
    pub static gsl_rng_random_bsd: *const gsl_rng_type;
    pub static gsl_rng_random_libc5: *const gsl_rng_type;
    pub static gsl_rng_random_glibc2: *const gsl_rng_type;
    pub static gsl_rng_rand48: *const gsl_rng_type;

    pub static gsl_rng_default: *const gsl_rng_type;
    pub static gsl_rng_ranf: *const gsl_rng_type;
    pub static gsl_rng_ranmar: *const gsl_rng_type;
    pub static gsl_rng_r250: *const gsl_rng_type;
    pub static gsl_rng_tt800: *const gsl_rng_type;
    pub static gsl_rng_vax: *const gsl_rng_type;
    pub static gsl_rng_transputer: *const gsl_rng_type;
    pub static gsl_rng_randu: *const gsl_rng_type;
    pub static gsl_rng_minstd: *const gsl_rng_type;
    pub static gsl_rng_uni: *const gsl_rng_type;
    pub static gsl_rng_uni32: *const gsl_rng_type;
    pub static gsl_rng_slatec: *const gsl_rng_type;
    pub static gsl_rng_zuf: *const gsl_rng_type;
    pub static gsl_rng_knuthran2: *const gsl_rng_type;
    pub static gsl_rng_knuthran2002: *const gsl_rng_type;
    pub static gsl_rng_knuthran: *const gsl_rng_type;
    pub static gsl_rng_borosh13: *const gsl_rng_type;
    pub static gsl_rng_fishman18: *const gsl_rng_type;
    pub static gsl_rng_fishman20: *const gsl_rng_type;
    pub static gsl_rng_lecuyer21: *const gsl_rng_type;
    pub static gsl_rng_waterman14: *const gsl_rng_type;
    pub static gsl_rng_fishman2x: *const gsl_rng_type;
    pub static gsl_rng_coveyou: *const gsl_rng_type;

    pub static gsl_rng_default_seed: c_ulong;

    pub static gsl_qrng_niederreiter_2: *const gsl_qrng_type;
    pub static gsl_qrng_sobol: *const gsl_qrng_type;
    pub static gsl_qrng_halton: *const gsl_qrng_type;
    pub static gsl_qrng_reversehalton: *const gsl_qrng_type;

    // Airy functions
    pub fn gsl_sf_airy_Ai(x: c_double, mode: ::Mode) -> c_double;
    pub fn gsl_sf_airy_Ai_e(x: c_double, mode: ::Mode, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_airy_Bi(x: c_double, mode: ::Mode) -> c_double;
    pub fn gsl_sf_airy_Bi_e(x: c_double, mode: ::Mode, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_airy_Ai_scaled(x: c_double, mode: ::Mode) -> c_double;
    pub fn gsl_sf_airy_Ai_scaled_e(x: c_double,
                                   mode: ::Mode,
                                   result: *mut gsl_sf_result)
                                   -> enums::Value;
    pub fn gsl_sf_airy_Bi_scaled(x: c_double, mode: ::Mode) -> c_double;
    pub fn gsl_sf_airy_Bi_scaled_e(x: c_double,
                                   mode: ::Mode,
                                   result: *mut gsl_sf_result)
                                   -> enums::Value;
    // Derivatives of Airy Functions
    pub fn gsl_sf_airy_Ai_deriv(x: c_double, mode: ::Mode) -> c_double;
    pub fn gsl_sf_airy_Ai_deriv_e(x: c_double,
                                  mode: ::Mode,
                                  result: *mut gsl_sf_result)
                                  -> enums::Value;
    pub fn gsl_sf_airy_Bi_deriv(x: c_double, mode: ::Mode) -> c_double;
    pub fn gsl_sf_airy_Bi_deriv_e(x: c_double,
                                  mode: ::Mode,
                                  result: *mut gsl_sf_result)
                                  -> enums::Value;
    pub fn gsl_sf_airy_Ai_deriv_scaled(x: c_double, mode: ::Mode) -> c_double;
    pub fn gsl_sf_airy_Ai_deriv_scaled_e(x: c_double,
                                         mode: ::Mode,
                                         result: *mut gsl_sf_result)
                                         -> enums::Value;
    pub fn gsl_sf_airy_Bi_deriv_scaled(x: c_double, mode: ::Mode) -> c_double;
    pub fn gsl_sf_airy_Bi_deriv_scaled_e(x: c_double,
                                         mode: ::Mode,
                                         result: *mut gsl_sf_result)
                                         -> enums::Value;
    //  Zeros of Airy Functions
    pub fn gsl_sf_airy_zero_Ai(s: c_uint) -> c_double;
    pub fn gsl_sf_airy_zero_Ai_e(s: c_uint, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_airy_zero_Bi(s: c_uint) -> c_double;
    pub fn gsl_sf_airy_zero_Bi_e(s: c_uint, result: *mut gsl_sf_result) -> enums::Value;
    // Zeros of Derivatives of Airy Functions
    pub fn gsl_sf_airy_zero_Ai_deriv(s: c_uint) -> c_double;
    pub fn gsl_sf_airy_zero_Ai_deriv_e(s: c_uint, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_airy_zero_Bi_deriv(s: c_uint) -> c_double;
    pub fn gsl_sf_airy_zero_Bi_deriv_e(s: c_uint, result: *mut gsl_sf_result) -> enums::Value;

    // Bessel functions
    // Regular Modified Cylindrical Bessel Functions
    pub fn gsl_sf_bessel_I0(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_I0_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_I1(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_I1_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_In(n: c_int, x: c_double) -> c_double;
    pub fn gsl_sf_bessel_In_e(n: c_int, x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_In_array(nmin: c_int,
                                  nmax: c_int,
                                  x: c_double,
                                  result_array: *mut c_double)
                                  -> enums::Value;
    pub fn gsl_sf_bessel_I0_scaled(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_I0_scaled_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_I1_scaled(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_I1_scaled_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_In_scaled(n: c_int, x: c_double) -> c_double;
    pub fn gsl_sf_bessel_In_scaled_e(n: c_int,
                                     x: c_double,
                                     result: *mut gsl_sf_result)
                                     -> enums::Value;
    pub fn gsl_sf_bessel_In_scaled_array(nmin: c_int,
                                         nmax: c_int,
                                         x: c_double,
                                         result_array: *mut c_double)
                                         -> enums::Value;
    // Regular Modified Spherical Bessel Functions
    // The regular modified spherical Bessel functions i_l(x) are related to the modified Bessel functions of fractional order, i_l(x) = \sqrt{\pi/(2x)} I_{l+1/2}(x)
    pub fn gsl_sf_bessel_i0_scaled(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_i0_scaled_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_i1_scaled(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_i1_scaled_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_i2_scaled(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_i2_scaled_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_il_scaled(l: c_int, x: c_double) -> c_double;
    pub fn gsl_sf_bessel_il_scaled_e(l: c_int,
                                     x: c_double,
                                     result: *mut gsl_sf_result)
                                     -> enums::Value;
    pub fn gsl_sf_bessel_il_scaled_array(lmax: c_int,
                                         x: c_double,
                                         result_array: *mut c_double)
                                         -> enums::Value;
    pub fn gsl_sf_bessel_Inu(nu: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_bessel_Inu_e(nu: c_double,
                               x: c_double,
                               result: *mut gsl_sf_result)
                               -> enums::Value;
    pub fn gsl_sf_bessel_Inu_scaled(nu: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_bessel_Inu_scaled_e(nu: c_double,
                                      x: c_double,
                                      result: *mut gsl_sf_result)
                                      -> enums::Value;
    // Regular Cylindrical Bessel Functions
    pub fn gsl_sf_bessel_J0(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_J0_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_J1(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_J1_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_Jn(n: c_int, x: c_double) -> c_double;
    pub fn gsl_sf_bessel_Jn_e(n: c_int, x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_Jn_array(nmin: c_int,
                                  nmax: c_int,
                                  x: c_double,
                                  result_array: *mut c_double)
                                  -> enums::Value;
    // Regular Spherical Bessel Functions
    pub fn gsl_sf_bessel_j0(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_j0_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_j1(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_j1_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_j2(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_j2_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_jl(l: c_int, x: c_double) -> c_double;
    pub fn gsl_sf_bessel_jl_e(l: c_int, x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_jl_array(lmax: c_int,
                                  x: c_double,
                                  result_array: *mut c_double)
                                  -> enums::Value;
    pub fn gsl_sf_bessel_jl_steed_array(lmax: c_int,
                                        x: c_double,
                                        result_array: *mut c_double)
                                        -> enums::Value;
    // Regular Bessel Function—Fractional Order
    pub fn gsl_sf_bessel_Jnu(nu: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_bessel_Jnu_e(nu: c_double,
                               x: c_double,
                               result: *mut gsl_sf_result)
                               -> enums::Value;
    pub fn gsl_sf_bessel_sequence_Jnu_e(nu: c_double,
                                        mode: ::Mode,
                                        size: i64,
                                        v: *mut c_double)
                                        -> enums::Value;
    // Irregular Modified Cylindrical Bessel Functions
    pub fn gsl_sf_bessel_K0(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_K0_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_K1(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_K1_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_Kn(n: c_int, x: c_double) -> c_double;
    pub fn gsl_sf_bessel_Kn_e(n: c_int, x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_Kn_array(nmin: c_int,
                                  nmax: c_int,
                                  x: c_double,
                                  result_array: *mut c_double)
                                  -> enums::Value;
    pub fn gsl_sf_bessel_K0_scaled(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_K0_scaled_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_K1_scaled(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_K1_scaled_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_Kn_scaled(n: c_int, x: c_double) -> c_double;
    pub fn gsl_sf_bessel_Kn_scaled_e(n: c_int,
                                     x: c_double,
                                     result: *mut gsl_sf_result)
                                     -> enums::Value;
    pub fn gsl_sf_bessel_Kn_scaled_array(nmin: c_int,
                                         nmax: c_int,
                                         x: c_double,
                                         result_array: *mut c_double)
                                         -> enums::Value;
    // Irregular Modified Spherical Bessel Functions
    pub fn gsl_sf_bessel_k0_scaled(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_k0_scaled_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_k1_scaled(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_k1_scaled_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_k2_scaled(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_k2_scaled_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_kl_scaled(l: c_int, x: c_double) -> c_double;
    pub fn gsl_sf_bessel_kl_scaled_e(l: c_int,
                                     x: c_double,
                                     result: *mut gsl_sf_result)
                                     -> enums::Value;
    pub fn gsl_sf_bessel_kl_scaled_array(lmax: c_int,
                                         x: c_double,
                                         result_array: *mut c_double)
                                         -> enums::Value;
    // Irregular Modified Bessel Functions—Fractional Order
    pub fn gsl_sf_bessel_Knu(nu: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_bessel_Knu_e(nu: c_double,
                               x: c_double,
                               result: *mut gsl_sf_result)
                               -> enums::Value;
    pub fn gsl_sf_bessel_lnKnu(nu: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_bessel_lnKnu_e(nu: c_double,
                                 x: c_double,
                                 result: *mut gsl_sf_result)
                                 -> enums::Value;
    pub fn gsl_sf_bessel_Knu_scaled(nu: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_bessel_Knu_scaled_e(nu: c_double,
                                      x: c_double,
                                      result: *mut gsl_sf_result)
                                      -> enums::Value;
    // Irregular Cylindrical Bessel Functions
    pub fn gsl_sf_bessel_Y0(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_Y0_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_Y1(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_Y1_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_Yn(n: c_int, x: c_double) -> c_double;
    pub fn gsl_sf_bessel_Yn_e(n: c_int, x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_Yn_array(nmin: c_int,
                                  nmax: c_int,
                                  x: c_double,
                                  result_array: *mut c_double)
                                  -> enums::Value;
    // Irregular Spherical Bessel Functions
    pub fn gsl_sf_bessel_y0(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_y0_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_y1(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_y1_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_y2(x: c_double) -> c_double;
    pub fn gsl_sf_bessel_y2_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_yl(l: c_int, x: c_double) -> c_double;
    pub fn gsl_sf_bessel_yl_e(l: c_int, x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_yl_array(lmax: c_int,
                                  x: c_double,
                                  result_array: *mut c_double)
                                  -> enums::Value;
    // Irregular Bessel Functions—Fractional Order
    pub fn gsl_sf_bessel_Ynu(nu: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_bessel_Ynu_e(nu: c_double,
                               x: c_double,
                               result: *mut gsl_sf_result)
                               -> enums::Value;
    // Zeros of Regular Bessel Functions
    pub fn gsl_sf_bessel_zero_J0(s: c_uint) -> c_double;
    pub fn gsl_sf_bessel_zero_J0_e(s: c_uint, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_zero_J1(s: c_uint) -> c_double;
    pub fn gsl_sf_bessel_zero_J1_e(s: c_uint, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_bessel_zero_Jnu(nu: c_double, s: c_uint) -> c_double;
    pub fn gsl_sf_bessel_zero_Jnu_e(nu: c_double,
                                    s: c_uint,
                                    result: *mut gsl_sf_result)
                                    -> enums::Value;

    // Trigonometric Functions
    pub fn gsl_sf_sin(x: c_double) -> c_double;
    pub fn gsl_sf_sin_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_cos(x: c_double) -> c_double;
    pub fn gsl_sf_cos_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_hypot(x: c_double) -> c_double;
    pub fn gsl_sf_hypot_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_sinc(x: c_double) -> c_double;
    pub fn gsl_sf_sinc_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_complex_sin_e(zr: c_double,
                                zi: c_double,
                                szr: *mut gsl_sf_result,
                                szi: *mut gsl_sf_result)
                                -> enums::Value;
    pub fn gsl_sf_complex_cos_e(zr: c_double,
                                zi: c_double,
                                czr: *mut gsl_sf_result,
                                czi: *mut gsl_sf_result)
                                -> enums::Value;
    pub fn gsl_sf_complex_logsin_e(zr: c_double,
                                   zi: c_double,
                                   lszr: *mut gsl_sf_result,
                                   lszi: *mut gsl_sf_result)
                                   -> enums::Value;
    pub fn gsl_sf_lnsinh(x: c_double) -> c_double;
    pub fn gsl_sf_lnsinh_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_lncosh(x: c_double) -> c_double;
    pub fn gsl_sf_lncosh_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_polar_to_rect(r: c_double,
                                theta: c_double,
                                x: *mut gsl_sf_result,
                                y: *mut gsl_sf_result)
                                -> enums::Value;
    pub fn gsl_sf_rect_to_polar(x: c_double,
                                y: c_double,
                                r: *mut gsl_sf_result,
                                theta: *mut gsl_sf_result)
                                -> enums::Value;
    pub fn gsl_sf_angle_restrict_symm(theta: c_double) -> c_double;
    pub fn gsl_sf_angle_restrict_symm_e(theta: *mut c_double) -> enums::Value;
    pub fn gsl_sf_angle_restrict_pos(theta: c_double) -> c_double;
    pub fn gsl_sf_angle_restrict_pos_e(theta: *mut c_double) -> enums::Value;
    pub fn gsl_sf_sin_err_e(x: c_double, dx: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_cos_err_e(x: c_double, dx: c_double, result: *mut gsl_sf_result) -> enums::Value;

    // Exponential Integrals functions
    pub fn gsl_sf_expint_E1(x: c_double) -> c_double;
    pub fn gsl_sf_expint_E1_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_expint_E2(x: c_double) -> c_double;
    pub fn gsl_sf_expint_E2_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_expint_En(n: c_int, x: c_double) -> c_double;
    pub fn gsl_sf_expint_En_e(n: c_int, x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_expint_Ei(x: c_double) -> c_double;
    pub fn gsl_sf_expint_Ei_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_Shi(x: c_double) -> c_double;
    pub fn gsl_sf_Shi_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_Chi(x: c_double) -> c_double;
    pub fn gsl_sf_Chi_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_expint_3(x: c_double) -> c_double;
    pub fn gsl_sf_expint_3_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_Si(x: c_double) -> c_double;
    pub fn gsl_sf_Si_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_Ci(x: c_double) -> c_double;
    pub fn gsl_sf_Ci_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_atanint(x: c_double) -> c_double;
    pub fn gsl_sf_atanint_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;

    // Clausen functions
    pub fn gsl_sf_clausen(x: c_double) -> c_double;
    pub fn gsl_sf_clausen_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;

    // Coulomb functions
    // Normalized Hydrogenic Bound States
    pub fn gsl_sf_hydrogenicR_1(Z: c_double, r: c_double) -> c_double;
    pub fn gsl_sf_hydrogenicR_1_e(Z: c_double,
                                  r: c_double,
                                  result: *mut gsl_sf_result)
                                  -> enums::Value;
    pub fn gsl_sf_hydrogenicR(n: c_int, l: c_int, Z: c_double, r: c_double) -> c_double;
    pub fn gsl_sf_hydrogenicR_e(n: c_int,
                                l: c_int,
                                Z: c_double,
                                r: c_double,
                                result: *mut gsl_sf_result)
                                -> enums::Value;
    // Coulomb Wave Functions
    // The Coulomb wave functions F_L(\eta,x), G_L(\eta,x) are described in Abramowitz & Stegun, Chapter 14. Because there can be a large dynamic range of values for these functions, overflows are handled gracefully. If an overflow occurs, GSL_EOVRFLW is signalled and exponent(s) are returned through the modifiable parameters exp_F, exp_G. The full solution can be reconstructed from the following relations,
    //
    // F_L(eta,x)  =  fc[k_L] * exp(exp_F)
    // G_L(eta,x)  =  gc[k_L] * exp(exp_G)
    //
    // F_L'(eta,x) = fcp[k_L] * exp(exp_F)
    // G_L'(eta,x) = gcp[k_L] * exp(exp_G)
    pub fn gsl_sf_coulomb_wave_FG_e(eta: c_double,
                                    x: c_double,
                                    L_F: c_double,
                                    k: c_int,
                                    F: *mut gsl_sf_result,
                                    Fp: *mut gsl_sf_result,
                                    G: *mut gsl_sf_result,
                                    Gp: *mut gsl_sf_result,
                                    exp_F: *mut c_double,
                                    exp_G: *mut c_double)
                                    -> enums::Value;
    pub fn gsl_sf_coulomb_wave_F_array(L_min: c_double,
                                       kmax: c_int,
                                       eta: c_double,
                                       x: c_double,
                                       fc_array: *mut c_double,
                                       F_exponent: *mut c_double)
                                       -> enums::Value;
    pub fn gsl_sf_coulomb_wave_FG_array(L_min: c_double,
                                        kmax: c_int,
                                        eta: c_double,
                                        x: c_double,
                                        fc_array: *mut c_double,
                                        gc_array: *mut c_double,
                                        F_exponent: *mut c_double,
                                        G_exponent: *mut c_double)
                                        -> enums::Value;
    pub fn gsl_sf_coulomb_wave_FGp_array(L_min: c_double,
                                         kmax: c_int,
                                         eta: c_double,
                                         x: c_double,
                                         fc_array: *mut c_double,
                                         fcp_array: *mut c_double,
                                         gc_array: *mut c_double,
                                         gcp_array: *mut c_double,
                                         F_exponent: *mut c_double,
                                         G_exponent: *mut c_double)
                                         -> enums::Value;
    pub fn gsl_sf_coulomb_wave_sphF_array(L_min: c_double,
                                          kmax: c_int,
                                          eta: c_double,
                                          x: c_double,
                                          fc_array: *mut c_double,
                                          f_exponent: *mut c_double)
                                          -> enums::Value;
    // Coulomb Wave Function Normalization Constant
    pub fn gsl_sf_coulomb_CL_e(L: c_double,
                               eta: c_double,
                               result: *mut gsl_sf_result)
                               -> enums::Value;
    pub fn gsl_sf_coulomb_CL_array(Lmin: c_double,
                                   kmax: c_int,
                                   eta: c_double,
                                   cl: *mut c_double)
                                   -> enums::Value;

    // Coupling Coefficients functions
    pub fn gsl_sf_coupling_3j(two_ja: c_int,
                              two_jb: c_int,
                              two_jc: c_int,
                              two_ma: c_int,
                              two_mc: c_int,
                              two_mc: c_int)
                              -> c_double;
    pub fn gsl_sf_coupling_3j_e(two_ja: c_int,
                                two_jb: c_int,
                                two_jc: c_int,
                                two_ma: c_int,
                                two_mc: c_int,
                                two_mc: c_int,
                                result: *mut gsl_sf_result)
                                -> enums::Value;
    pub fn gsl_sf_coupling_6j(two_ja: c_int,
                              two_jb: c_int,
                              two_jc: c_int,
                              two_jd: c_int,
                              two_je: c_int,
                              two_jf: c_int)
                              -> c_double;
    pub fn gsl_sf_coupling_6j_e(two_ja: c_int,
                                two_jb: c_int,
                                two_jc: c_int,
                                two_jd: c_int,
                                two_je: c_int,
                                two_jf: c_int,
                                result: *mut gsl_sf_result)
                                -> enums::Value;
    pub fn gsl_sf_coupling_9j(two_ja: c_int,
                              two_jb: c_int,
                              two_jc: c_int,
                              two_jd: c_int,
                              two_je: c_int,
                              two_jf: c_int,
                              two_jg: c_int,
                              two_jh: c_int,
                              two_ji: c_int)
                              -> c_double;
    pub fn gsl_sf_coupling_9j_e(two_ja: c_int,
                                two_jb: c_int,
                                two_jc: c_int,
                                two_jd: c_int,
                                two_je: c_int,
                                two_jf: c_int,
                                two_jg: c_int,
                                two_jh: c_int,
                                two_ji: c_int,
                                result: *mut gsl_sf_result)
                                -> enums::Value;

    // Dawson functions
    pub fn gsl_sf_dawson(x: c_double) -> c_double;
    pub fn gsl_sf_dawson_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;

    // Debye functions
    pub fn gsl_sf_debye_1(x: c_double) -> c_double;
    pub fn gsl_sf_debye_1_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_debye_2(x: c_double) -> c_double;
    pub fn gsl_sf_debye_2_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_debye_3(x: c_double) -> c_double;
    pub fn gsl_sf_debye_3_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_debye_4(x: c_double) -> c_double;
    pub fn gsl_sf_debye_4_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_debye_5(x: c_double) -> c_double;
    pub fn gsl_sf_debye_5_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_debye_6(x: c_double) -> c_double;
    pub fn gsl_sf_debye_6_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;

    // Dilogarithm functions
    // real argument
    pub fn gsl_sf_dilog(x: c_double) -> c_double;
    pub fn gsl_sf_dilog_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    // complex argument
    pub fn gsl_sf_complex_dilog_e(r: c_double,
                                  theta: c_double,
                                  result: *mut gsl_sf_result,
                                  result_im: *mut gsl_sf_result)
                                  -> enums::Value;

    // Elementary Operations functions
    pub fn gsl_sf_multiply_e(x: c_double, y: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_multiply_err_e(x: c_double,
                                 dx: c_double,
                                 y: c_double,
                                 dy: c_double,
                                 result: *mut gsl_sf_result)
                                 -> enums::Value;

    // Elliptic functions (Jacobi)
    pub fn gsl_sf_elljac_e(u: c_double,
                           m: c_double,
                           sn: *mut c_double,
                           cn: *mut c_double,
                           dn: *mut c_double)
                           -> enums::Value;

    // Error functions
    pub fn gsl_sf_erf(x: c_double) -> c_double;
    pub fn gsl_sf_erf_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    // Complementary Error functions
    pub fn gsl_sf_erfc(x: c_double) -> c_double;
    pub fn gsl_sf_erfc_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    // Log Complementary Error functions
    pub fn gsl_sf_log_erfc(x: c_double) -> c_double;
    pub fn gsl_sf_log_erfc_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    // Probability functions
    // The probability functions for the Normal or Gaussian distribution are described in Abramowitz & Stegun, Section 26.2.
    pub fn gsl_sf_erf_Z(x: c_double) -> c_double;
    pub fn gsl_sf_erf_Z_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_erf_Q(x: c_double) -> c_double;
    pub fn gsl_sf_erf_Q_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_hazard(x: c_double) -> c_double;
    pub fn gsl_sf_hazard_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;

    // Exponential functions
    pub fn gsl_sf_exp(x: c_double) -> c_double;
    pub fn gsl_sf_exp_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_exp_e10_e(x: c_double, result: *mut gsl_sf_result_e10) -> enums::Value;
    pub fn gsl_sf_exp_mult(x: c_double, y: c_double) -> c_double;
    pub fn gsl_sf_exp_mult_e(x: c_double, y: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_exp_mult_e10_e(x: c_double,
                                 y: c_double,
                                 result: *mut gsl_sf_result_e10)
                                 -> enums::Value;
    // Relative Exponential functions
    pub fn gsl_sf_expm1(x: c_double) -> c_double;
    pub fn gsl_sf_expm1_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_exprel(x: c_double) -> c_double;
    pub fn gsl_sf_exprel_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_exprel_2(x: c_double) -> c_double;
    pub fn gsl_sf_exprel_2_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_exprel_n(n: c_int, x: c_double) -> c_double;
    pub fn gsl_sf_exprel_n_e(n: c_int, x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    // Exponentiation With Error Estimate
    pub fn gsl_sf_exp_err_e(x: c_double, dx: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_exp_err_e10_e(x: c_double,
                                dx: c_double,
                                result: *mut gsl_sf_result_e10)
                                -> enums::Value;
    pub fn gsl_sf_exp_mult_err_e(x: c_double,
                                 dx: c_double,
                                 y: c_double,
                                 dy: c_double,
                                 result: *mut gsl_sf_result)
                                 -> enums::Value;
    pub fn gsl_sf_exp_mult_err_e10_e(x: c_double,
                                     dx: c_double,
                                     y: c_double,
                                     dy: c_double,
                                     result: *mut gsl_sf_result_e10)
                                     -> enums::Value;

    // Gamma Beta functions
    // Gamma functions
    pub fn gsl_sf_gamma(x: c_double) -> c_double;
    pub fn gsl_sf_gamma_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_lngamma(x: c_double) -> c_double;
    pub fn gsl_sf_lngamma_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_lngamma_sgn_e(x: c_double,
                                result_lg: *mut gsl_sf_result,
                                sgn: *mut c_double)
                                -> enums::Value;
    pub fn gsl_sf_gammastar(x: c_double) -> c_double;
    pub fn gsl_sf_gammastar_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_gammainv(x: c_double) -> c_double;
    pub fn gsl_sf_gammainv_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_lngamma_complex_e(zr: c_double,
                                    zi: c_double,
                                    lnr: *mut gsl_sf_result,
                                    arg: *mut gsl_sf_result)
                                    -> enums::Value;
    // Factorials
    pub fn gsl_sf_fact(n: c_uint) -> c_double;
    pub fn gsl_sf_fact_e(n: c_uint, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_doublefact(n: c_uint) -> c_double;
    pub fn gsl_sf_doublefact_e(n: c_uint, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_lnfact(n: c_uint) -> c_double;
    pub fn gsl_sf_lnfact_e(n: c_uint, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_lndoublefact(n: c_uint) -> c_double;
    pub fn gsl_sf_lndoublefact_e(n: c_uint, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_choose(n: c_uint, m: c_uint) -> c_double;
    pub fn gsl_sf_choose_e(n: c_uint, m: c_uint, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_lnchoose(n: c_uint, m: c_uint) -> c_double;
    pub fn gsl_sf_lnchoose_e(n: c_uint, m: c_uint, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_taylorcoeff(n: c_int, x: c_double) -> c_double;
    pub fn gsl_sf_taylorcoeff_e(n: c_int, x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    // Pochhammer Symbol
    pub fn gsl_sf_poch(a: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_poch_e(a: c_double, x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_lnpoch(a: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_lnpoch_e(a: c_double, x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_lnpoch_sgn_e(a: c_double,
                               x: c_double,
                               result: *mut gsl_sf_result,
                               sgn: *mut c_double)
                               -> enums::Value;
    pub fn gsl_sf_pochrel(a: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_pochrel_e(a: c_double, x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    // Beta functions
    pub fn gsl_sf_beta(a: c_double, b: c_double) -> c_double;
    pub fn gsl_sf_beta_e(a: c_double, b: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_lnbeta(a: c_double, b: c_double) -> c_double;
    pub fn gsl_sf_lnbeta_e(a: c_double, b: c_double, result: *mut gsl_sf_result) -> enums::Value;
    // Incomplete Gamma functions
    pub fn gsl_sf_gamma_inc(a: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_gamma_inc_e(a: c_double,
                              x: c_double,
                              result: *mut gsl_sf_result)
                              -> enums::Value;
    pub fn gsl_sf_gamma_inc_Q(a: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_gamma_inc_Q_e(a: c_double,
                                x: c_double,
                                result: *mut gsl_sf_result)
                                -> enums::Value;
    pub fn gsl_sf_gamma_inc_P(a: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_gamma_inc_P_e(a: c_double,
                                x: c_double,
                                result: *mut gsl_sf_result)
                                -> enums::Value;
    // Incomplete Beta functions
    pub fn gsl_sf_beta_inc(a: c_double, b: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_beta_inc_e(a: c_double,
                             b: c_double,
                             x: c_double,
                             result: *mut gsl_sf_result)
                             -> enums::Value;

    // Gegenbauer functions
    pub fn gsl_sf_gegenpoly_1(lambda: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_gegenpoly_2(lambda: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_gegenpoly_3(lambda: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_gegenpoly_1_e(lambda: c_double,
                                x: c_double,
                                result: *mut gsl_sf_result)
                                -> enums::Value;
    pub fn gsl_sf_gegenpoly_2_e(lambda: c_double,
                                x: c_double,
                                result: *mut gsl_sf_result)
                                -> enums::Value;
    pub fn gsl_sf_gegenpoly_3_e(lambda: c_double,
                                x: c_double,
                                result: *mut gsl_sf_result)
                                -> enums::Value;
    pub fn gsl_sf_gegenpoly_n(n: c_int, lambda: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_gegenpoly_n_e(n: c_int,
                                lambda: c_double,
                                x: c_double,
                                result: *mut gsl_sf_result)
                                -> enums::Value;
    pub fn gsl_sf_gegenpoly_array(nmax: c_int,
                                  lambda: c_double,
                                  x: c_double,
                                  result_array: *mut c_double)
                                  -> enums::Value;

    // Hypergeometric functions
    pub fn gsl_sf_hyperg_0F1(c: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_hyperg_0F1_e(c: c_double,
                               x: c_double,
                               result: *mut gsl_sf_result)
                               -> enums::Value;
    pub fn gsl_sf_hyperg_1F1_int(m: c_int, n: c_int, x: c_double) -> c_double;
    pub fn gsl_sf_hyperg_1F1_int_e(m: c_int,
                                   n: c_int,
                                   x: c_double,
                                   result: *mut gsl_sf_result)
                                   -> enums::Value;
    pub fn gsl_sf_hyperg_1F1(a: c_double, b: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_hyperg_1F1_e(a: c_double,
                               b: c_double,
                               x: c_double,
                               result: *mut gsl_sf_result)
                               -> enums::Value;
    pub fn gsl_sf_hyperg_U_int(m: c_int, n: c_int, x: c_double) -> c_double;
    pub fn gsl_sf_hyperg_U_int_e(m: c_int,
                                 n: c_int,
                                 x: c_double,
                                 result: *mut gsl_sf_result)
                                 -> enums::Value;
    pub fn gsl_sf_hyperg_U_int_e10_e(m: c_int,
                                     n: c_int,
                                     x: c_double,
                                     result: *mut gsl_sf_result_e10)
                                     -> enums::Value;
    pub fn gsl_sf_hyperg_U(a: c_double, b: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_hyperg_U_e(a: c_double,
                             b: c_double,
                             x: c_double,
                             result: *mut gsl_sf_result)
                             -> enums::Value;
    pub fn gsl_sf_hyperg_U_e10_e(a: c_double,
                                 b: c_double,
                                 x: c_double,
                                 result: *mut gsl_sf_result_e10)
                                 -> enums::Value;
    pub fn gsl_sf_hyperg_2F1(a: c_double, b: c_double, c: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_hyperg_2F1_e(a: c_double,
                               b: c_double,
                               c: c_double,
                               x: c_double,
                               result: *mut gsl_sf_result)
                               -> enums::Value;
    pub fn gsl_sf_hyperg_2F1_conj(aR: c_double,
                                  aI: c_double,
                                  c: c_double,
                                  x: c_double)
                                  -> c_double;
    pub fn gsl_sf_hyperg_2F1_conj_e(aR: c_double,
                                    aI: c_double,
                                    c: c_double,
                                    x: c_double,
                                    result: *mut gsl_sf_result)
                                    -> enums::Value;
    pub fn gsl_sf_hyperg_2F1_renorm(a: c_double,
                                    b: c_double,
                                    c: c_double,
                                    x: c_double)
                                    -> c_double;
    pub fn gsl_sf_hyperg_2F1_renorm_e(a: c_double,
                                      b: c_double,
                                      c: c_double,
                                      x: c_double,
                                      result: *mut gsl_sf_result)
                                      -> enums::Value;
    pub fn gsl_sf_hyperg_2F1_conj_renorm(aR: c_double,
                                         aI: c_double,
                                         c: c_double,
                                         x: c_double)
                                         -> c_double;
    pub fn gsl_sf_hyperg_2F1_conj_renorm_e(aR: c_double,
                                           aI: c_double,
                                           c: c_double,
                                           x: c_double,
                                           result: *mut gsl_sf_result)
                                           -> enums::Value;
    pub fn gsl_sf_hyperg_2F0(a: c_double, b: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_hyperg_2F0_e(a: c_double,
                               b: c_double,
                               x: c_double,
                               result: *mut gsl_sf_result)
                               -> enums::Value;

    /// Laguerre functions
    pub fn gsl_sf_laguerre_1(a: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_laguerre_2(a: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_laguerre_3(a: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_laguerre_1_e(a: c_double,
                               x: c_double,
                               result: *mut gsl_sf_result)
                               -> enums::Value;
    pub fn gsl_sf_laguerre_2_e(a: c_double,
                               x: c_double,
                               result: *mut gsl_sf_result)
                               -> enums::Value;
    pub fn gsl_sf_laguerre_3_e(a: c_double,
                               x: c_double,
                               result: *mut gsl_sf_result)
                               -> enums::Value;
    pub fn gsl_sf_laguerre_n(n: c_int, a: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_laguerre_n_e(n: c_int,
                               a: c_double,
                               x: c_double,
                               result: *mut gsl_sf_result)
                               -> enums::Value;

    // Lambert W functions
    pub fn gsl_sf_lambert_W0(x: c_double) -> c_double;
    pub fn gsl_sf_lambert_W0_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_lambert_Wm1(x: c_double) -> c_double;
    pub fn gsl_sf_lambert_Wm1_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;

    // Legendre functions
    // Legendre Polynomials
    pub fn gsl_sf_legendre_P1(x: c_double) -> c_double;
    pub fn gsl_sf_legendre_P2(x: c_double) -> c_double;
    pub fn gsl_sf_legendre_P3(x: c_double) -> c_double;
    pub fn gsl_sf_legendre_P1_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_legendre_P2_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_legendre_P3_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_legendre_Pl(l: c_int, x: c_double) -> c_double;
    pub fn gsl_sf_legendre_Pl_e(l: c_int, x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_legendre_Pl_array(lmax: c_int,
                                    x: c_double,
                                    result_array: *mut c_double)
                                    -> enums::Value;
    pub fn gsl_sf_legendre_Pl_deriv_array(lmax: c_int,
                                          x: c_double,
                                          result_array: *mut c_double,
                                          result_deriv_array: *mut c_double)
                                          -> enums::Value;
    pub fn gsl_sf_legendre_Q0(x: c_double) -> c_double;
    pub fn gsl_sf_legendre_Q0_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_legendre_Q1(x: c_double) -> c_double;
    pub fn gsl_sf_legendre_Q1_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_legendre_Ql(l: c_int, x: c_double) -> c_double;
    pub fn gsl_sf_legendre_Ql_e(l: c_int, x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    // Associated Legendre Polynomials and Spherical Harmonics
    pub fn gsl_sf_legendre_Plm(l: c_int, m: c_int, x: c_double) -> c_double;
    pub fn gsl_sf_legendre_Plm_e(l: c_int,
                                 m: c_int,
                                 x: c_double,
                                 result: *mut gsl_sf_result)
                                 -> enums::Value;
    #[cfg(not(feature = "v2"))]
    pub fn gsl_sf_legendre_Plm_array(lmax: c_int,
                                     m: c_int,
                                     x: c_double,
                                     result_array: *mut c_double)
                                     -> enums::Value;
    #[cfg(not(feature = "v2"))]
    pub fn gsl_sf_legendre_Plm_deriv_array(lmax: c_int,
                                           m: c_int,
                                           x: c_double,
                                           result_array: *mut c_double,
                                           result_deriv_array: *mut c_double)
                                           -> enums::Value;
    pub fn gsl_sf_legendre_sphPlm(l: c_int, m: c_int, x: c_double) -> c_double;
    pub fn gsl_sf_legendre_sphPlm_e(l: c_int,
                                    m: c_int,
                                    x: c_double,
                                    result: *mut gsl_sf_result)
                                    -> enums::Value;
    #[cfg(not(feature = "v2"))]
    pub fn gsl_sf_legendre_sphPlm_array(lmax: c_int,
                                        m: c_int,
                                        x: c_double,
                                        result_array: *mut c_double)
                                        -> enums::Value;
    #[cfg(not(feature = "v2"))]
    pub fn gsl_sf_legendre_sphPlm_deriv_array(lmax: c_int,
                                              m: c_int,
                                              x: c_double,
                                              result_array: *mut c_double,
                                              result_deriv_array: *mut c_double)
                                              -> enums::Value;
    #[cfg(not(feature = "v2"))]
    pub fn gsl_sf_legendre_array_size(lmax: c_int, m: c_int) -> enums::Value;
    // Conical functions
    pub fn gsl_sf_conicalP_half(lambda: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_conicalP_half_e(lambda: c_double,
                                  x: c_double,
                                  result: *mut gsl_sf_result)
                                  -> enums::Value;
    pub fn gsl_sf_conicalP_mhalf(lambda: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_conicalP_mhalf_e(lambda: c_double,
                                   x: c_double,
                                   result: *mut gsl_sf_result)
                                   -> enums::Value;
    pub fn gsl_sf_conicalP_0(lambda: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_conicalP_0_e(lambda: c_double,
                               x: c_double,
                               result: *mut gsl_sf_result)
                               -> enums::Value;
    pub fn gsl_sf_conicalP_1(lambda: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_conicalP_1_e(lambda: c_double,
                               x: c_double,
                               result: *mut gsl_sf_result)
                               -> enums::Value;
    pub fn gsl_sf_conicalP_sph_reg(l: c_int, lambda: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_conicalP_sph_reg_e(l: c_int,
                                     lambda: c_double,
                                     x: c_double,
                                     result: *mut gsl_sf_result)
                                     -> enums::Value;
    pub fn gsl_sf_conicalP_cyl_reg(m: c_int, lambda: c_double, x: c_double) -> c_double;
    pub fn gsl_sf_conicalP_cyl_reg_e(m: c_int,
                                     lambda: c_double,
                                     x: c_double,
                                     result: *mut gsl_sf_result)
                                     -> enums::Value;
    // Radial Functions for Hyperbolic Space
    pub fn gsl_sf_legendre_H3d_0(lambda: c_double, eta: c_double) -> c_double;
    pub fn gsl_sf_legendre_H3d_0_e(lambda: c_double,
                                   eta: c_double,
                                   result: *mut gsl_sf_result)
                                   -> enums::Value;
    pub fn gsl_sf_legendre_H3d_1(lambda: c_double, eta: c_double) -> c_double;
    pub fn gsl_sf_legendre_H3d_1_e(lambda: c_double,
                                   eta: c_double,
                                   result: *mut gsl_sf_result)
                                   -> enums::Value;
    pub fn gsl_sf_legendre_H3d(l: c_int, lambda: c_double, eta: c_double) -> c_double;
    pub fn gsl_sf_legendre_H3d_e(l: c_int,
                                 lambda: c_double,
                                 eta: c_double,
                                 result: *mut gsl_sf_result)
                                 -> enums::Value;
    pub fn gsl_sf_legendre_H3d_array(lmax: c_int,
                                     lambda: c_double,
                                     eta: c_double,
                                     result_array: *mut c_double)
                                     -> enums::Value;

    // Logarithm and Related Functions
    pub fn gsl_sf_log(x: c_double) -> c_double;
    pub fn gsl_sf_log_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_log_abs(x: c_double) -> c_double;
    pub fn gsl_sf_log_abs_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_complex_log_e(zr: c_double,
                                zi: c_double,
                                lnr: *mut gsl_sf_result,
                                theta: *mut gsl_sf_result)
                                -> enums::Value;
    pub fn gsl_sf_log_1plusx(x: c_double) -> c_double;
    pub fn gsl_sf_log_1plusx_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_log_1plusx_mx(x: c_double) -> c_double;
    pub fn gsl_sf_log_1plusx_mx_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;

    // Power functions
    pub fn gsl_sf_pow_int(x: c_double, n: c_int) -> c_double;
    pub fn gsl_sf_pow_int_e(x: c_double, n: c_int, result: *mut gsl_sf_result) -> enums::Value;

    // Psi (Digamma) functions
    // Digamma functions
    pub fn gsl_sf_psi_int(n: c_int) -> c_double;
    pub fn gsl_sf_psi_int_e(n: c_int, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_psi(x: c_double) -> c_double;
    pub fn gsl_sf_psi_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_psi_1piy(y: c_double) -> c_double;
    pub fn gsl_sf_psi_1piy_e(y: c_double, result: *mut gsl_sf_result) -> enums::Value;
    // Trigamma functions
    pub fn gsl_sf_psi_1_int(n: c_int) -> c_double;
    pub fn gsl_sf_psi_1_int_e(n: c_int, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_psi_1(x: c_double) -> c_double;
    pub fn gsl_sf_psi_1_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    // Polygamma functions
    pub fn gsl_sf_psi_n(n: c_int, x: c_double) -> c_double;
    pub fn gsl_sf_psi_n_e(n: c_int, x: c_double, result: *mut gsl_sf_result) -> enums::Value;

    // Synchrotron functions
    pub fn gsl_sf_synchrotron_1(x: c_double) -> c_double;
    pub fn gsl_sf_synchrotron_1_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_synchrotron_2(x: c_double) -> c_double;
    pub fn gsl_sf_synchrotron_2_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;

    // Transport functions
    pub fn gsl_sf_transport_2(x: c_double) -> c_double;
    pub fn gsl_sf_transport_2_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_transport_3(x: c_double) -> c_double;
    pub fn gsl_sf_transport_3_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_transport_4(x: c_double) -> c_double;
    pub fn gsl_sf_transport_4_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_transport_5(x: c_double) -> c_double;
    pub fn gsl_sf_transport_5_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;

    // Zeta functions
    // Riemann Zeta functions
    pub fn gsl_sf_zeta_int(n: c_int) -> c_double;
    pub fn gsl_sf_zeta_int_e(n: c_int, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_zeta(s: c_double) -> c_double;
    pub fn gsl_sf_zeta_e(s: c_double, result: *mut gsl_sf_result) -> enums::Value;
    // Riemann Zeta functions Minus One
    pub fn gsl_sf_zetam1_int(n: c_int) -> c_double;
    pub fn gsl_sf_zetam1_int_e(n: c_int, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_zetam1(s: c_double) -> c_double;
    pub fn gsl_sf_zetam1_e(s: c_double, result: *mut gsl_sf_result) -> enums::Value;
    // Hurwitz Zeta functions
    pub fn gsl_sf_hzeta(s: c_double, q: c_double) -> c_double;
    pub fn gsl_sf_hzeta_e(s: c_double, q: c_double, result: *mut gsl_sf_result) -> enums::Value;
    // Eta functions
    pub fn gsl_sf_eta_int(n: c_int) -> c_double;
    pub fn gsl_sf_eta_int_e(n: c_int, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_eta(s: c_double) -> c_double;
    pub fn gsl_sf_eta_e(s: c_double, result: *mut gsl_sf_result) -> enums::Value;

    // Elliptic Integrals
    // Legendre Form of Complete Elliptic Integrals
    pub fn gsl_sf_ellint_Kcomp(k: c_double, mode: ::Mode) -> c_double;
    pub fn gsl_sf_ellint_Kcomp_e(k: c_double,
                                 mode: ::Mode,
                                 result: *mut gsl_sf_result)
                                 -> enums::Value;
    pub fn gsl_sf_ellint_Ecomp(k: c_double, mode: ::Mode) -> c_double;
    pub fn gsl_sf_ellint_Ecomp_e(k: c_double,
                                 mode: ::Mode,
                                 result: *mut gsl_sf_result)
                                 -> enums::Value;
    pub fn gsl_sf_ellint_Pcomp(k: c_double, n: c_double, mode: ::Mode) -> c_double;
    pub fn gsl_sf_ellint_Pcomp_e(k: c_double,
                                 n: c_double,
                                 mode: ::Mode,
                                 result: *mut gsl_sf_result)
                                 -> enums::Value;
    // Legendre Form of Incomplete Elliptic Integrals
    pub fn gsl_sf_ellint_F(phi: c_double, k: c_double, mode: ::Mode) -> c_double;
    pub fn gsl_sf_ellint_F_e(phi: c_double,
                             k: c_double,
                             mode: ::Mode,
                             result: *mut gsl_sf_result)
                             -> enums::Value;
    pub fn gsl_sf_ellint_E(phi: c_double, k: c_double, mode: ::Mode) -> c_double;
    pub fn gsl_sf_ellint_E_e(phi: c_double,
                             k: c_double,
                             mode: ::Mode,
                             result: *mut gsl_sf_result)
                             -> enums::Value;
    pub fn gsl_sf_ellint_P(phi: c_double, k: c_double, n: c_double, mode: ::Mode) -> c_double;
    pub fn gsl_sf_ellint_P_e(phi: c_double,
                             k: c_double,
                             n: c_double,
                             mode: ::Mode,
                             result: *mut gsl_sf_result)
                             -> enums::Value;
    #[cfg(feature = "v2")]
    pub fn gsl_sf_ellint_D(phi: c_double, k: c_double, mode: ::Mode) -> c_double;
    #[cfg(not(feature = "v2"))]
    pub fn gsl_sf_ellint_D(phi: c_double, k: c_double, n: c_double, mode: ::Mode) -> c_double;
    pub fn gsl_sf_ellint_D_e(phi: c_double,
                             k: c_double,
                             n: c_double,
                             mode: ::Mode,
                             result: *mut gsl_sf_result)
                             -> enums::Value;
    // Carlson Forms
    pub fn gsl_sf_ellint_RC(x: c_double, y: c_double, mode: ::Mode) -> c_double;
    pub fn gsl_sf_ellint_RC_e(x: c_double,
                              y: c_double,
                              mode: ::Mode,
                              result: *mut gsl_sf_result)
                              -> enums::Value;
    pub fn gsl_sf_ellint_RD(x: c_double, y: c_double, z: c_double, mode: ::Mode) -> c_double;
    pub fn gsl_sf_ellint_RD_e(x: c_double,
                              y: c_double,
                              z: c_double,
                              mode: ::Mode,
                              result: *mut gsl_sf_result)
                              -> enums::Value;
    pub fn gsl_sf_ellint_RF(x: c_double, y: c_double, z: c_double, mode: ::Mode) -> c_double;
    pub fn gsl_sf_ellint_RF_e(x: c_double,
                              y: c_double,
                              z: c_double,
                              mode: ::Mode,
                              result: *mut gsl_sf_result)
                              -> enums::Value;
    pub fn gsl_sf_ellint_RJ(x: c_double,
                            y: c_double,
                            z: c_double,
                            p: c_double,
                            mode: ::Mode)
                            -> c_double;
    pub fn gsl_sf_ellint_RJ_e(x: c_double,
                              y: c_double,
                              z: c_double,
                              p: c_double,
                              mode: ::Mode,
                              result: *mut gsl_sf_result)
                              -> enums::Value;

    // Fermi-Dirac functions
    // Complete Fermi-Dirac Integrals
    pub fn gsl_sf_fermi_dirac_m1(x: c_double) -> c_double;
    pub fn gsl_sf_fermi_dirac_m1_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_fermi_dirac_0(x: c_double) -> c_double;
    pub fn gsl_sf_fermi_dirac_0_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_fermi_dirac_1(x: c_double) -> c_double;
    pub fn gsl_sf_fermi_dirac_1_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_fermi_dirac_2(x: c_double) -> c_double;
    pub fn gsl_sf_fermi_dirac_2_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_fermi_dirac_int(j: c_int, x: c_double) -> c_double;
    pub fn gsl_sf_fermi_dirac_int_e(j: c_int,
                                    x: c_double,
                                    result: *mut gsl_sf_result)
                                    -> enums::Value;
    pub fn gsl_sf_fermi_dirac_mhalf(x: c_double) -> c_double;
    pub fn gsl_sf_fermi_dirac_mhalf_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_fermi_dirac_half(x: c_double) -> c_double;
    pub fn gsl_sf_fermi_dirac_half_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_fermi_dirac_3half(x: c_double) -> c_double;
    pub fn gsl_sf_fermi_dirac_3half_e(x: c_double, result: *mut gsl_sf_result) -> enums::Value;
    // Incomplete Fermi-Dirac Integrals
    pub fn gsl_sf_fermi_dirac_inc_0(x: c_double, b: c_double) -> c_double;
    pub fn gsl_sf_fermi_dirac_inc_0_e(x: c_double,
                                      b: c_double,
                                      result: *mut gsl_sf_result)
                                      -> enums::Value;

    // Elementary functions
    pub fn gsl_log1p(x: c_double) -> c_double;
    pub fn gsl_expm1(x: c_double) -> c_double;
    pub fn gsl_hypot(x: c_double, y: c_double) -> c_double;
    pub fn gsl_hypot3(x: c_double, y: c_double, z: c_double) -> c_double;
    pub fn gsl_acosh(x: c_double) -> c_double;
    pub fn gsl_asinh(x: c_double) -> c_double;
    pub fn gsl_atanh(x: c_double) -> c_double;
    pub fn gsl_ldexp(x: c_double, e: c_int) -> c_double;
    pub fn gsl_frexp(x: c_double, e: *mut c_int) -> c_double;

    // Mathieu functions
    // Mathieu functions Workspace
    pub fn gsl_sf_mathieu_alloc(n: size_t, qmax: c_double) -> *mut gsl_sf_mathieu_workspace;
    pub fn gsl_sf_mathieu_free(work: *mut gsl_sf_mathieu_workspace);
    // Mathieu functions Characteristic Values
    pub fn gsl_sf_mathieu_a(n: c_int, q: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_mathieu_b(n: c_int, q: c_double, result: *mut gsl_sf_result) -> enums::Value;
    pub fn gsl_sf_mathieu_a_array(order_min: c_int,
                                  order_max: c_int,
                                  q: c_double,
                                  work: *mut gsl_sf_mathieu_workspace,
                                  result_array: *mut c_double)
                                  -> enums::Value;
    pub fn gsl_sf_mathieu_b_array(order_min: c_int,
                                  order_max: c_int,
                                  q: c_double,
                                  work: *mut gsl_sf_mathieu_workspace,
                                  result_array: *mut c_double)
                                  -> enums::Value;
    // Angular Mathieu functions
    pub fn gsl_sf_mathieu_ce(n: c_int,
                             q: c_double,
                             x: c_double,
                             result: *mut gsl_sf_result)
                             -> enums::Value;
    pub fn gsl_sf_mathieu_se(n: c_int,
                             q: c_double,
                             x: c_double,
                             result: *mut gsl_sf_result)
                             -> enums::Value;
    pub fn gsl_sf_mathieu_ce_array(nmin: c_int,
                                   nmax: c_int,
                                   q: c_double,
                                   x: c_double,
                                   work: *mut gsl_sf_mathieu_workspace,
                                   result_array: *mut c_double)
                                   -> enums::Value;
    pub fn gsl_sf_mathieu_se_array(nmin: c_int,
                                   nmax: c_int,
                                   q: c_double,
                                   x: c_double,
                                   work: *mut gsl_sf_mathieu_workspace,
                                   result_array: *mut c_double)
                                   -> enums::Value;
    // Radial Mathieu functions
    pub fn gsl_sf_mathieu_Mc(j: c_int,
                             n: c_int,
                             q: c_double,
                             x: c_double,
                             result: *mut gsl_sf_result)
                             -> enums::Value;
    pub fn gsl_sf_mathieu_Ms(j: c_int,
                             n: c_int,
                             q: c_double,
                             x: c_double,
                             result: *mut gsl_sf_result)
                             -> enums::Value;
    pub fn gsl_sf_mathieu_Mc_array(j: c_int,
                                   nmin: c_int,
                                   nmax: c_int,
                                   q: c_double,
                                   x: c_double,
                                   work: *mut gsl_sf_mathieu_workspace,
                                   result_array: *mut c_double)
                                   -> enums::Value;
    pub fn gsl_sf_mathieu_Ms_array(j: c_int,
                                   nmin: c_int,
                                   nmax: c_int,
                                   q: c_double,
                                   x: c_double,
                                   work: *mut gsl_sf_mathieu_workspace,
                                   result_array: *mut c_double)
                                   -> enums::Value;

    // Complex number functions
    // https://www.gnu.org/software/gsl/manual/html_node/Representation-of-complex-numbers.html#Representation-of-complex-numbers
    pub fn gsl_complex_rect(x: c_double, y: c_double) -> gsl_complex;
    pub fn gsl_complex_polar(r: c_double, theta: c_double) -> gsl_complex;
    pub fn gsl_complex_arg(z: gsl_complex) -> c_double;
    pub fn gsl_complex_abs(z: gsl_complex) -> c_double;
    pub fn gsl_complex_abs2(z: gsl_complex) -> c_double;
    pub fn gsl_complex_logabs(z: gsl_complex) -> c_double;
    pub fn gsl_complex_add(a: gsl_complex, b: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_sub(a: gsl_complex, b: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_mul(a: gsl_complex, b: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_div(a: gsl_complex, b: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_add_real(a: gsl_complex, x: c_double) -> gsl_complex;
    pub fn gsl_complex_sub_real(a: gsl_complex, x: c_double) -> gsl_complex;
    pub fn gsl_complex_mul_real(a: gsl_complex, x: c_double) -> gsl_complex;
    pub fn gsl_complex_div_real(a: gsl_complex, x: c_double) -> gsl_complex;
    pub fn gsl_complex_add_imag(a: gsl_complex, x: c_double) -> gsl_complex;
    pub fn gsl_complex_sub_imag(a: gsl_complex, x: c_double) -> gsl_complex;
    pub fn gsl_complex_mul_imag(a: gsl_complex, x: c_double) -> gsl_complex;
    pub fn gsl_complex_div_imag(a: gsl_complex, x: c_double) -> gsl_complex;
    pub fn gsl_complex_conjugate(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_inverse(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_negative(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_sqrt(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_sqrt_real(x: c_double) -> gsl_complex;
    pub fn gsl_complex_pow(z: gsl_complex, a: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_pow_real(z: gsl_complex, x: c_double) -> gsl_complex;
    pub fn gsl_complex_exp(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_log(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_log10(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_log_b(z: gsl_complex, b: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_sin(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_cos(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_tan(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_sec(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_csc(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_cot(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_arcsin(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_arcsin_real(z: c_double) -> gsl_complex;
    pub fn gsl_complex_arccos(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_arccos_real(z: c_double) -> gsl_complex;
    pub fn gsl_complex_arctan(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_arcsec(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_arcsec_real(z: c_double) -> gsl_complex;
    pub fn gsl_complex_arccsc(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_arccsc_real(z: c_double) -> gsl_complex;
    pub fn gsl_complex_arccot(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_sinh(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_cosh(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_tanh(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_sech(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_csch(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_coth(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_arcsinh(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_arccosh(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_arccosh_real(z: c_double) -> gsl_complex;
    pub fn gsl_complex_arctanh(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_arctanh_real(z: c_double) -> gsl_complex;
    pub fn gsl_complex_arcsech(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_arccsch(z: gsl_complex) -> gsl_complex;
    pub fn gsl_complex_arccoth(z: gsl_complex) -> gsl_complex;

    // ComplexFloat number functions
    /*
    pub fn gsl_complex_float_arg(z: gsl_complex_float) -> c_float;
    pub fn gsl_complex_float_abs(z: gsl_complex_float) -> c_float;
    pub fn gsl_complex_float_abs2(z: gsl_complex_float) -> c_float;
    pub fn gsl_complex_float_logabs(z: gsl_complex_float) -> c_float;
    pub fn gsl_complex_float_add(a: gsl_complex_float, b: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_sub(a: gsl_complex_float, b: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_mul(a: gsl_complex_float, b: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_div(a: gsl_complex_float, b: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_add_real(a: gsl_complex_float, x: c_float) -> gsl_complex_float;
    pub fn gsl_complex_float_sub_real(a: gsl_complex_float, x: c_float) -> gsl_complex_float;
    pub fn gsl_complex_float_mul_real(a: gsl_complex_float, x: c_float) -> gsl_complex_float;
    pub fn gsl_complex_float_div_real(a: gsl_complex_float, x: c_float) -> gsl_complex_float;
    pub fn gsl_complex_float_add_imag(a: gsl_complex_float, x: c_float) -> gsl_complex_float;
    pub fn gsl_complex_float_sub_imag(a: gsl_complex_float, x: c_float) -> gsl_complex_float;
    pub fn gsl_complex_float_mul_imag(a: gsl_complex_float, x: c_float) -> gsl_complex_float;
    pub fn gsl_complex_float_div_imag(a: gsl_complex_float, x: c_float) -> gsl_complex_float;
    pub fn gsl_complex_float_conjugate(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_inverse(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_negative(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_sqrt(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_sqrt_real(x: c_float) -> gsl_complex_float;
    pub fn gsl_complex_float_pow(z: gsl_complex_float, a: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_pow_real(z: gsl_complex_float, x: c_float) -> gsl_complex_float;
    pub fn gsl_complex_float_exp(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_log(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_log10(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_log_b(z: gsl_complex_float, b: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_sin(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_cos(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_tan(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_sec(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_csc(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_cot(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_arcsin(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_arcsin_real(z: c_float) -> gsl_complex_float;
    pub fn gsl_complex_float_arccos(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_arccos_real(z: c_float) -> gsl_complex_float;
    pub fn gsl_complex_float_arctan(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_arcsec(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_arcsec_real(z: c_float) -> gsl_complex_float;
    pub fn gsl_complex_float_arccsc(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_arccsc_real(z: c_float) -> gsl_complex_float;
    pub fn gsl_complex_float_arccot(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_sinh(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_cosh(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_tanh(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_sech(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_csch(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_coth(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_arcsinh(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_arccosh(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_arccosh_real(z: c_float) -> gsl_complex_float;
    pub fn gsl_complex_float_arctanh(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_arctanh_real(z: c_float) -> gsl_complex_float;
    pub fn gsl_complex_float_arcsech(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_arccsch(z: gsl_complex_float) -> gsl_complex_float;
    pub fn gsl_complex_float_arccoth(z: gsl_complex_float) -> gsl_complex_float;
    */

    // Pow functions
    pub fn gsl_pow_int(x: c_double, n: c_int) -> c_double;
    pub fn gsl_pow_uint(x: c_double, n: c_uint) -> c_double;
    pub fn gsl_pow_2(x: c_double) -> c_double;
    pub fn gsl_pow_3(x: c_double) -> c_double;
    pub fn gsl_pow_4(x: c_double) -> c_double;
    pub fn gsl_pow_5(x: c_double) -> c_double;
    pub fn gsl_pow_6(x: c_double) -> c_double;
    pub fn gsl_pow_7(x: c_double) -> c_double;
    pub fn gsl_pow_8(x: c_double) -> c_double;
    pub fn gsl_pow_9(x: c_double) -> c_double;

    // Random Number Generation
    pub fn gsl_rng_alloc(T: *const gsl_rng_type) -> *mut gsl_rng;
    pub fn gsl_rng_set(r: *mut gsl_rng, s: c_ulong);
    pub fn gsl_rng_free(r: *mut gsl_rng);
    pub fn gsl_rng_get(r: *mut gsl_rng) -> c_ulong;
    pub fn gsl_rng_uniform(r: *mut gsl_rng) -> c_double;
    pub fn gsl_rng_uniform_pos(r: *mut gsl_rng) -> c_double;
    pub fn gsl_rng_uniform_int(r: *mut gsl_rng, n: c_ulong) -> c_ulong;
    pub fn gsl_rng_name(r: *const gsl_rng) -> *const c_char;
    pub fn gsl_rng_max(r: *const gsl_rng) -> c_ulong;
    pub fn gsl_rng_min(r: *const gsl_rng) -> c_ulong;
    pub fn gsl_rng_state(r: *const gsl_rng) -> *mut c_void;
    pub fn gsl_rng_size(r: *const gsl_rng) -> size_t;
    pub fn gsl_rng_types_setup() -> *const *mut gsl_rng_type;
    pub fn gsl_rng_memcpy(dest: *mut gsl_rng, src: *const gsl_rng) -> enums::Value;
    pub fn gsl_rng_clone(r: *const gsl_rng) -> *mut gsl_rng;
    pub fn gsl_rng_env_setup() -> *const gsl_rng_type;

    // Error function
    #[allow(dead_code)]
    pub fn gsl_error(reason: *const c_char, file: *const c_char, line: c_int, gsl_errno: c_int);

    // Quasi-Random Sequences
    // Quasi-random number generator initialization
    pub fn gsl_qrng_alloc(t: *const gsl_qrng_type, d: c_uint) -> *mut gsl_qrng;
    pub fn gsl_qrng_free(q: *mut gsl_qrng);
    pub fn gsl_qrng_init(q: *mut gsl_qrng);
    // Sampling from a quasi-random number generator
    pub fn gsl_qrng_get(q: *const gsl_qrng, x: *mut c_double) -> enums::Value;
    // Auxiliary quasi-random number generator functions
    pub fn gsl_qrng_name(q: *const gsl_qrng) -> *const c_char;
    pub fn gsl_qrng_size(q: *const gsl_qrng) -> size_t;
    pub fn gsl_qrng_state(q: *const gsl_qrng) -> *mut c_void;
    // Saving and resorting quasi-random number generator state
    pub fn gsl_qrng_memcpy(dest: *mut gsl_qrng, src: *const gsl_qrng) -> enums::Value;
    pub fn gsl_qrng_clone(q: *const gsl_qrng) -> *mut gsl_qrng;

    // Series Acceleration
    // Acceleration functions
    pub fn gsl_sum_levin_u_alloc(n: size_t) -> *mut gsl_sum_levin_u_workspace;
    pub fn gsl_sum_levin_u_free(w: *mut gsl_sum_levin_u_workspace);
    pub fn gsl_sum_levin_u_accel(array: *const c_double,
                                 array_size: size_t,
                                 w: *mut gsl_sum_levin_u_workspace,
                                 sum_accel: *mut c_double,
                                 abserr: *mut c_double)
                                 -> enums::Value;
    // Acceleration functions without error estimation
    pub fn gsl_sum_levin_utrunc_alloc(n: size_t) -> *mut gsl_sum_levin_utrunc_workspace;
    pub fn gsl_sum_levin_utrunc_free(w: *mut gsl_sum_levin_utrunc_workspace);
    pub fn gsl_sum_levin_utrunc_accel(array: *const c_double,
                                      array_size: size_t,
                                      w: *mut gsl_sum_levin_utrunc_workspace,
                                      sum_accel: *mut c_double,
                                      abserr_trunc: *mut c_double)
                                      -> enums::Value;
}

#[repr(C)]
pub struct gsl_sf_result {
    pub val: c_double,
    pub err: c_double,
}

#[repr(C)]
pub struct gsl_sf_result_e10 {
    pub val: c_double,
    pub err: c_double,
    pub e10: c_int,
}

#[repr(C)]
pub struct gsl_complex {
    pub dat: [c_double; 2],
}

#[repr(C)]
pub struct gsl_complex_float {
    pub dat: [c_float; 2],
}

#[repr(C)]
pub struct gsl_sf_mathieu_workspace {
    pub size: size_t,
    pub even_order: size_t,
    pub odd_order: size_t,
    pub extra_values: c_int,
    pub qa: c_double, // allow for caching of results: not implemented yet
    pub qb: c_double, // allow for caching of results: not implemented yet
    pub aa: *mut c_double,
    pub bb: *mut c_double,
    pub dd: *mut c_double,
    pub ee: *mut c_double,
    pub tt: *mut c_double,
    pub e2: *mut c_double,
    pub zz: *mut c_double,
    pub eval: *mut gsl_vector,
    pub evec: *mut gsl_matrix,
    pub wmat: *mut gsl_eigen_symmv_workspace,
}

pub type rng_set = Option<extern "C" fn(state: *mut c_void, seed: c_ulong)>;
pub type rng_get = Option<extern "C" fn(state: *mut c_void) -> c_ulong>;
pub type rng_get_double = Option<extern "C" fn(state: *mut c_void) -> c_double>;

#[repr(C)]
pub struct gsl_rng_type {
    pub name: *const c_char,
    pub max: c_ulong,
    pub min: c_ulong,
    pub size: size_t,
    pub set: rng_set,
    pub get: rng_get,
    pub get_double: rng_get_double,
}

#[repr(C)]
pub struct gsl_rng {
    pub _type: *const gsl_rng_type,
    pub state: *mut c_void,
}

#[repr(C)]
pub struct gsl_ran_discrete_t {
    pub K: size_t,
    pub A: *mut size_t,
    pub F: *mut c_double,
}

// Structure describing a generator instance of a specified type, with generator-specific state info and dimension-specific info.
#[repr(C)]
pub struct gsl_qrng {
    pub type_: *const gsl_qrng_type,
    pub dimension: c_uint,
    pub state_size: size_t,
    pub state: *mut c_void,
}

// Structure describing a type of generator.
#[repr(C)]
pub struct gsl_qrng_type {
    pub name: *const c_char,
    pub max_dimension: c_uint,
    pub state_size: Option<extern "C" fn(dimension: c_uint) -> size_t>,
    pub init_state: Option<extern "C" fn(state: *mut c_void, dimension: c_uint) -> enums::Value>,
    pub get: Option<extern "C" fn(state: *mut c_void, dimension: c_uint, x: *mut c_double)
                                  -> enums::Value>,
}

/*
 * size        = number of terms the workspace can handle
 * sum_plain   = simple sum of series
 * q_num       = backward diagonal of numerator; length = size
 * q_den       = backward diagonal of denominator; length = size
 * dq_num      = table of numerator derivatives; length = size**2
 * dq_den      = table of denominator derivatives; length = size**2
 * dsum        = derivative of sum wrt term i; length = size
*/
#[repr(C)]
pub struct gsl_sum_levin_u_workspace {
    pub size: size_t,
    // position in array
    pub i: size_t,
    // number of calls
    pub terms_used: size_t,
    pub sum_plain: c_double,
    pub q_num: *mut c_double,
    pub q_den: *mut c_double,
    pub dq_num: *mut c_double,
    pub dq_den: *mut c_double,
    pub dsum: *mut c_double,
}

#[repr(C)]
pub struct gsl_sum_levin_utrunc_workspace {
    pub size: size_t,
    // position in array
    pub i: size_t,
    // number of calls
    pub terms_used: size_t,
    pub sum_plain: c_double,
    pub q_num: *mut c_double,
    pub q_den: *mut c_double,
    pub dsum: *mut c_double,
}
