//! Fast Fourier Transforms


use libc::{c_double, size_t};

use super::{gsl_complex, gsl_complex_packed_array};
use enums;

extern "C" {
    // Radix-2 FFT routines for complex data
    pub fn gsl_fft_complex_radix2_forward(data: gsl_complex_packed_array,
                                          stride: size_t,
                                          n: size_t)
                                          -> enums::Value;
    pub fn gsl_fft_complex_radix2_transform(data: gsl_complex_packed_array,
                                            stride: size_t,
                                            n: size_t,
                                            sign: ::FftDirection)
                                            -> enums::Value;
    pub fn gsl_fft_complex_radix2_backward(data: gsl_complex_packed_array,
                                           stride: size_t,
                                           n: size_t)
                                           -> enums::Value;
    pub fn gsl_fft_complex_radix2_inverse(data: gsl_complex_packed_array,
                                          stride: size_t,
                                          n: size_t)
                                          -> enums::Value;
    pub fn gsl_fft_complex_radix2_dif_forward(data: gsl_complex_packed_array,
                                              stride: size_t,
                                              n: size_t)
                                              -> enums::Value;
    pub fn gsl_fft_complex_radix2_dif_transform(data: gsl_complex_packed_array,
                                                stride: size_t,
                                                n: size_t,
                                                sign: ::FftDirection)
                                                -> enums::Value;
    pub fn gsl_fft_complex_radix2_dif_backward(data: gsl_complex_packed_array,
                                               stride: size_t,
                                               n: size_t)
                                               -> enums::Value;
    pub fn gsl_fft_complex_radix2_dif_inverse(data: gsl_complex_packed_array,
                                              stride: size_t,
                                              n: size_t)
                                              -> enums::Value;
    // Mixed-radix FFT routines for complex data
    pub fn gsl_fft_complex_wavetable_alloc(n: size_t) -> *mut gsl_fft_complex_wavetable;
    pub fn gsl_fft_complex_wavetable_free(w: *mut gsl_fft_complex_wavetable);
    pub fn gsl_fft_complex_workspace_alloc(n: size_t) -> *mut gsl_fft_complex_workspace;
    pub fn gsl_fft_complex_workspace_free(w: *mut gsl_fft_complex_workspace);
    pub fn gsl_fft_complex_forward(data: gsl_complex_packed_array,
                                   stride: size_t,
                                   n: size_t,
                                   wavetable: *const gsl_fft_complex_wavetable,
                                   work: *mut gsl_fft_complex_workspace)
                                   -> enums::Value;
    pub fn gsl_fft_complex_transform(data: gsl_complex_packed_array,
                                     stride: size_t,
                                     n: size_t,
                                     wavetable: *const gsl_fft_complex_wavetable,
                                     work: *mut gsl_fft_complex_workspace,
                                     sign: ::FftDirection)
                                     -> enums::Value;
    pub fn gsl_fft_complex_backward(data: gsl_complex_packed_array,
                                    stride: size_t,
                                    n: size_t,
                                    wavetable: *const gsl_fft_complex_wavetable,
                                    work: *mut gsl_fft_complex_workspace)
                                    -> enums::Value;
    pub fn gsl_fft_complex_inverse(data: gsl_complex_packed_array,
                                   stride: size_t,
                                   n: size_t,
                                   wavetable: *const gsl_fft_complex_wavetable,
                                   work: *mut gsl_fft_complex_workspace)
                                   -> enums::Value;
    // Radix-2 FFT routines for real data
    pub fn gsl_fft_real_radix2_transform(data: *mut c_double,
                                         stride: size_t,
                                         n: size_t)
                                         -> enums::Value;
    pub fn gsl_fft_halfcomplex_radix2_inverse(data: *mut c_double,
                                              stride: size_t,
                                              n: size_t)
                                              -> enums::Value;
    pub fn gsl_fft_halfcomplex_radix2_backward(data: *mut c_double,
                                               stride: size_t,
                                               n: size_t)
                                               -> enums::Value;
    pub fn gsl_fft_halfcomplex_radix2_unpack(halfcomplex_coefficient: *mut c_double,
                                             complex_coefficient: gsl_complex_packed_array,
                                             stride: size_t,
                                             n: size_t)
                                             -> enums::Value;
}

#[repr(C)]
pub struct gsl_fft_complex_wavetable {
    pub n: size_t,
    pub nf: size_t,
    pub factor: [size_t; 64],
    pub twiddle: [*mut gsl_complex; 64],
    pub trig: *mut gsl_complex,
}

#[repr(C)]
pub struct gsl_fft_complex_workspace {
    pub n: size_t,
    pub scratch: *mut c_double,
}
