//! Wavelet Transforms

use libc::{c_double, size_t, c_char};

use super::gsl_matrix;
use enums;

extern "C" {
    pub static gsl_wavelet_daubechies: *const gsl_wavelet_type;
    pub static gsl_wavelet_daubechies_centered: *const gsl_wavelet_type;
    pub static gsl_wavelet_haar: *const gsl_wavelet_type;
    pub static gsl_wavelet_haar_centered: *const gsl_wavelet_type;
    pub static gsl_wavelet_bspline: *const gsl_wavelet_type;
    pub static gsl_wavelet_bspline_centered: *const gsl_wavelet_type;

    // Initialization
    pub fn gsl_wavelet_alloc(t: *const gsl_wavelet_type, k: size_t) -> *mut gsl_wavelet;
    pub fn gsl_wavelet_name(w: *const gsl_wavelet) -> *const c_char;
    pub fn gsl_wavelet_free(w: *mut gsl_wavelet);
    pub fn gsl_wavelet_workspace_alloc(n: size_t) -> *mut gsl_wavelet_workspace;
    pub fn gsl_wavelet_workspace_free(w: *mut gsl_wavelet_workspace);
    // Wavelet transforms in one dimension
    pub fn gsl_wavelet_transform(w: *const gsl_wavelet,
                                 data: *mut c_double,
                                 stride: size_t,
                                 n: size_t,
                                 dir: ::WaveletDirection,
                                 work: *mut gsl_wavelet_workspace)
                                 -> enums::Value;
    pub fn gsl_wavelet_transform_forward(w: *const gsl_wavelet,
                                         data: *mut c_double,
                                         stride: size_t,
                                         n: size_t,
                                         work: *mut gsl_wavelet_workspace)
                                         -> enums::Value;
    pub fn gsl_wavelet_transform_inverse(w: *const gsl_wavelet,
                                         data: *mut c_double,
                                         stride: size_t,
                                         n: size_t,
                                         work: *mut gsl_wavelet_workspace)
                                         -> enums::Value;
    // Wavelet transforms in two dimension
    pub fn gsl_wavelet2d_transform(w: *const gsl_wavelet,
                                   data: *mut c_double,
                                   tda: size_t,
                                   size1: size_t,
                                   size2: size_t,
                                   dir: ::WaveletDirection,
                                   work: *mut gsl_wavelet_workspace)
                                   -> enums::Value;
    pub fn gsl_wavelet2d_transform_forward(w: *const gsl_wavelet,
                                           data: *mut c_double,
                                           tda: size_t,
                                           size1: size_t,
                                           size2: size_t,
                                           work: *mut gsl_wavelet_workspace)
                                           -> enums::Value;
    pub fn gsl_wavelet2d_transform_inverse(w: *const gsl_wavelet,
                                           data: *mut c_double,
                                           tda: size_t,
                                           size1: size_t,
                                           size2: size_t,
                                           work: *mut gsl_wavelet_workspace)
                                           -> enums::Value;
    pub fn gsl_wavelet2d_transform_matrix(w: *const gsl_wavelet,
                                          m: *mut gsl_matrix,
                                          dir: ::WaveletDirection,
                                          work: *mut gsl_wavelet_workspace)
                                          -> enums::Value;
    pub fn gsl_wavelet2d_transform_matrix_forward(w: *const gsl_wavelet,
                                                  m: *mut gsl_matrix,
                                                  work: *mut gsl_wavelet_workspace)
                                                  -> enums::Value;
    pub fn gsl_wavelet2d_transform_matrix_inverse(w: *const gsl_wavelet,
                                                  m: *mut gsl_matrix,
                                                  work: *mut gsl_wavelet_workspace)
                                                  -> enums::Value;
    pub fn gsl_wavelet2d_nstransform(w: *const gsl_wavelet,
                                     data: *mut c_double,
                                     tda: size_t,
                                     size1: size_t,
                                     size2: size_t,
                                     dir: ::WaveletDirection,
                                     work: *mut gsl_wavelet_workspace)
                                     -> enums::Value;
    pub fn gsl_wavelet2d_nstransform_forward(w: *const gsl_wavelet,
                                             data: *mut c_double,
                                             tda: size_t,
                                             size1: size_t,
                                             size2: size_t,
                                             work: *mut gsl_wavelet_workspace)
                                             -> enums::Value;
    pub fn gsl_wavelet2d_nstransform_inverse(w: *const gsl_wavelet,
                                             data: *mut c_double,
                                             tda: size_t,
                                             size1: size_t,
                                             size2: size_t,
                                             work: *mut gsl_wavelet_workspace)
                                             -> enums::Value;
    pub fn gsl_wavelet2d_nstransform_matrix(w: *const gsl_wavelet,
                                            m: *mut gsl_matrix,
                                            dir: ::WaveletDirection,
                                            work: *mut gsl_wavelet_workspace)
                                            -> enums::Value;
    pub fn gsl_wavelet2d_nstransform_matrix_forward(w: *const gsl_wavelet,
                                                    m: *mut gsl_matrix,
                                                    work: *mut gsl_wavelet_workspace)
                                                    -> enums::Value;
    pub fn gsl_wavelet2d_nstransform_matrix_inverse(w: *const gsl_wavelet,
                                                    m: *mut gsl_matrix,
                                                    work: *mut gsl_wavelet_workspace)
                                                    -> enums::Value;
}

#[repr(C)]
pub struct gsl_wavelet_workspace {
    pub scratch: *mut c_double,
    pub n: size_t,
}

#[repr(C)]
pub struct gsl_wavelet {
    pub type_: *const gsl_wavelet_type,
    pub h1: *const c_double,
    pub g1: *const c_double,
    pub h2: *const c_double,
    pub g2: *const c_double,
    pub nc: size_t,
    pub offset: size_t,
}

#[repr(C)]
pub struct gsl_wavelet_type {
    pub name: *const c_char,
    pub init: Option<extern "C" fn(h1: *const *const c_double,
                                   g1: *const *const c_double,
                                   h2: *const *const c_double,
                                   g2: *const *const c_double,
                                   nc: *mut size_t,
                                   offset: *mut size_t,
                                   member: size_t)
                                   -> enums::Value>,
}
