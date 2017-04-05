//! Histograms functions

use libc::{c_double, c_int, size_t};

use enums;

extern "C" {
    // Histogram allocation
    pub fn gsl_histogram_alloc(n: size_t) -> *mut gsl_histogram;
    pub fn gsl_histogram_set_ranges(h: *mut gsl_histogram,
                                    range: *const c_double,
                                    size: size_t)
                                    -> enums::Value;
    pub fn gsl_histogram_set_ranges_uniform(h: *mut gsl_histogram,
                                            xmin: c_double,
                                            xmax: c_double)
                                            -> enums::Value;
    pub fn gsl_histogram_free(h: *mut gsl_histogram);
    // Copying Histograms
    pub fn gsl_histogram_memcpy(dest: *mut gsl_histogram,
                                src: *const gsl_histogram)
                                -> enums::Value;
    pub fn gsl_histogram_clone(src: *const gsl_histogram) -> *mut gsl_histogram;
    // Updating and accessing histogram elements
    pub fn gsl_histogram_increment(h: *mut gsl_histogram, x: c_double) -> enums::Value;
    pub fn gsl_histogram_accumulate(h: *mut gsl_histogram,
                                    x: c_double,
                                    weigth: c_double)
                                    -> enums::Value;
    pub fn gsl_histogram_get(h: *const gsl_histogram, i: size_t) -> c_double;
    pub fn gsl_histogram_get_range(h: *const gsl_histogram,
                                   i: size_t,
                                   lower: *mut c_double,
                                   upper: *mut c_double)
                                   -> enums::Value;
    pub fn gsl_histogram_max(h: *const gsl_histogram) -> c_double;
    pub fn gsl_histogram_min(h: *const gsl_histogram) -> c_double;
    pub fn gsl_histogram_bins(h: *const gsl_histogram) -> size_t;
    pub fn gsl_histogram_reset(h: *mut gsl_histogram);
    pub fn gsl_histogram_find(h: *const gsl_histogram,
                              x: c_double,
                              i: *mut size_t)
                              -> enums::Value;
    pub fn gsl_histogram_max_val(h: *const gsl_histogram) -> c_double;
    pub fn gsl_histogram_max_bin(h: *const gsl_histogram) -> size_t;
    pub fn gsl_histogram_min_val(h: *const gsl_histogram) -> c_double;
    pub fn gsl_histogram_min_bin(h: *const gsl_histogram) -> size_t;
    pub fn gsl_histogram_mean(h: *const gsl_histogram) -> c_double;
    pub fn gsl_histogram_sigma(h: *const gsl_histogram) -> c_double;
    pub fn gsl_histogram_sum(h: *const gsl_histogram) -> c_double;
    pub fn gsl_histogram_equal_bins_p(h1: *const gsl_histogram, h2: *const gsl_histogram) -> c_int;
    pub fn gsl_histogram_add(h1: *mut gsl_histogram, h2: *const gsl_histogram) -> enums::Value;
    pub fn gsl_histogram_sub(h1: *mut gsl_histogram, h2: *const gsl_histogram) -> enums::Value;
    pub fn gsl_histogram_mul(h1: *mut gsl_histogram, h2: *const gsl_histogram) -> enums::Value;
    pub fn gsl_histogram_div(h1: *mut gsl_histogram, h2: *const gsl_histogram) -> enums::Value;
    pub fn gsl_histogram_scale(h1: *mut gsl_histogram, scale: c_double) -> enums::Value;
    pub fn gsl_histogram_shift(h1: *mut gsl_histogram, offset: c_double) -> enums::Value;
    // The histogram probability distribution struct
    pub fn gsl_histogram_pdf_alloc(n: size_t) -> *mut gsl_histogram_pdf;
    pub fn gsl_histogram_pdf_init(p: *mut gsl_histogram_pdf,
                                  h: *const gsl_histogram)
                                  -> enums::Value;
    pub fn gsl_histogram_pdf_free(p: *mut gsl_histogram_pdf);
    pub fn gsl_histogram_pdf_sample(p: *const gsl_histogram_pdf, r: c_double) -> c_double;
    // 2D Histogram allocation
    pub fn gsl_histogram2d_alloc(nx: size_t, ny: size_t) -> *mut gsl_histogram2d;
    pub fn gsl_histogram2d_set_ranges(h: *mut gsl_histogram2d,
                                      xrange: *const c_double,
                                      xsize: size_t,
                                      yrange: *const c_double,
                                      ysize: size_t)
                                      -> enums::Value;
    pub fn gsl_histogram2d_set_ranges_uniform(h: *mut gsl_histogram2d,
                                              xmin: c_double,
                                              xmax: c_double,
                                              ymin: c_double,
                                              ymax: c_double)
                                              -> enums::Value;
    pub fn gsl_histogram2d_free(h: *mut gsl_histogram2d);
    pub fn gsl_histogram2d_memcpy(dest: *mut gsl_histogram2d,
                                  src: *const gsl_histogram2d)
                                  -> enums::Value;
    pub fn gsl_histogram2d_clone(src: *const gsl_histogram2d) -> *mut gsl_histogram2d;
    pub fn gsl_histogram2d_increment(h: *mut gsl_histogram2d,
                                     x: c_double,
                                     y: c_double)
                                     -> enums::Value;
    pub fn gsl_histogram2d_accumulate(h: *mut gsl_histogram2d,
                                      x: c_double,
                                      y: c_double,
                                      weight: c_double)
                                      -> enums::Value;
    pub fn gsl_histogram2d_get(h: *const gsl_histogram2d, i: size_t, j: size_t) -> c_double;
    pub fn gsl_histogram2d_get_xrange(h: *const gsl_histogram2d,
                                      i: size_t,
                                      xlower: *mut c_double,
                                      xupper: *mut c_double)
                                      -> enums::Value;
    pub fn gsl_histogram2d_get_yrange(h: *const gsl_histogram2d,
                                      j: size_t,
                                      ylower: *mut c_double,
                                      yupper: *mut c_double)
                                      -> enums::Value;
    pub fn gsl_histogram2d_xmax(h: *const gsl_histogram2d) -> c_double;
    pub fn gsl_histogram2d_xmin(h: *const gsl_histogram2d) -> c_double;
    pub fn gsl_histogram2d_nx(h: *const gsl_histogram2d) -> size_t;
    pub fn gsl_histogram2d_ymax(h: *const gsl_histogram2d) -> c_double;
    pub fn gsl_histogram2d_ymin(h: *const gsl_histogram2d) -> c_double;
    pub fn gsl_histogram2d_ny(h: *const gsl_histogram2d) -> size_t;
    pub fn gsl_histogram2d_reset(h: *mut gsl_histogram2d);
    pub fn gsl_histogram2d_find(h: *const gsl_histogram2d,
                                x: c_double,
                                y: c_double,
                                i: *mut size_t,
                                j: *mut size_t)
                                -> enums::Value;
    // 2D Histogram Statistics
    pub fn gsl_histogram2d_max_val(h: *const gsl_histogram2d) -> c_double;
    pub fn gsl_histogram2d_max_bin(h: *const gsl_histogram2d, i: *mut size_t, j: *mut size_t);
    pub fn gsl_histogram2d_min_val(h: *const gsl_histogram2d) -> c_double;
    pub fn gsl_histogram2d_min_bin(h: *const gsl_histogram2d, i: *mut size_t, j: *mut size_t);
    pub fn gsl_histogram2d_xmean(h: *const gsl_histogram2d) -> c_double;
    pub fn gsl_histogram2d_ymean(h: *const gsl_histogram2d) -> c_double;
    pub fn gsl_histogram2d_xsigma(h: *const gsl_histogram2d) -> c_double;
    pub fn gsl_histogram2d_ysigma(h: *const gsl_histogram2d) -> c_double;
    pub fn gsl_histogram2d_cov(h: *const gsl_histogram2d) -> c_double;
    pub fn gsl_histogram2d_sum(h: *const gsl_histogram2d) -> c_double;
    pub fn gsl_histogram2d_equal_bins_p(h1: *const gsl_histogram2d,
                                        h2: *const gsl_histogram2d)
                                        -> c_int;
    pub fn gsl_histogram2d_add(h1: *mut gsl_histogram2d,
                               h2: *const gsl_histogram2d)
                               -> enums::Value;
    pub fn gsl_histogram2d_sub(h1: *mut gsl_histogram2d,
                               h2: *const gsl_histogram2d)
                               -> enums::Value;
    pub fn gsl_histogram2d_mul(h1: *mut gsl_histogram2d,
                               h2: *const gsl_histogram2d)
                               -> enums::Value;
    pub fn gsl_histogram2d_div(h1: *mut gsl_histogram2d,
                               h2: *const gsl_histogram2d)
                               -> enums::Value;
    pub fn gsl_histogram2d_scale(h1: *mut gsl_histogram2d, scale: c_double) -> enums::Value;
    pub fn gsl_histogram2d_shift(h1: *mut gsl_histogram2d, offset: c_double) -> enums::Value;
    // Resampling from 2D histograms
    pub fn gsl_histogram2d_pdf_alloc(nx: size_t, ny: size_t) -> *mut gsl_histogram2d_pdf;
    pub fn gsl_histogram2d_pdf_init(p: *mut gsl_histogram2d_pdf,
                                    h: *const gsl_histogram2d)
                                    -> enums::Value;
    pub fn gsl_histogram2d_pdf_free(p: *mut gsl_histogram2d_pdf);
    pub fn gsl_histogram2d_pdf_sample(p: *const gsl_histogram2d_pdf,
                                      r1: c_double,
                                      r2: c_double,
                                      x: *mut c_double,
                                      y: *mut c_double)
                                      -> enums::Value;
}


#[repr(C)]
pub struct gsl_histogram {
    pub n: size_t, // This is the number of histogram bins
    pub range: *mut c_double, // The ranges of the bins are stored in an array of n+1 elements pointed to by range.
    pub bin: *mut c_double, /* The counts for each bin are stored in an array of n elements pointed to by bin. The bins are floating-point numbers, so you can increment them by non-integer values if necessary.

The range for bin[i] is given by range[i] to range[i+1]. For n bins there are n+1 entries in the array range. Each bin is inclusive at the lower end and exclusive at the upper end. Mathematically this means that the bins are defined by the following inequality,

bin[i] corresponds to range[i] <= x < range[i+1]
Here is a diagram of the correspondence between ranges and bins on the number-line for x,

     [ bin[0] )[ bin[1] )[ bin[2] )[ bin[3] )[ bin[4] )
  ---|---------|---------|---------|---------|---------|---  x
   r[0]      r[1]      r[2]      r[3]      r[4]      r[5]

In this picture the values of the range array are denoted by r. On the left-hand side of each bin the square bracket ‘[’ denotes an inclusive lower bound (r <= x), and the round parentheses ‘)’ on the right-hand side denote an exclusive upper bound (x < r). Thus any samples which fall on the upper end of the histogram are excluded. If you want to include this value for the last bin you will need to add an extra bin to your histogram.
*/
}

#[repr(C)]
pub struct gsl_histogram_pdf {
    pub n: size_t, // This is the number of bins used to approximate the probability distribution function.
    pub range: *mut c_double, // The ranges of the bins are stored in an array of n+1 elements pointed to by range.
    pub sum: *mut c_double, // The cumulative probability for the bins is stored in an array of n elements pointed to by sum.
}

#[repr(C)]
pub struct gsl_histogram2d {
    pub nx: size_t, // This is the number of histogram bins in the x direction.
    pub ny: size_t, // This is the number of histogram bins in the y direction.
    pub xrange: *mut c_double, // The ranges of the bins in the x-direction are stored in an array of nx + 1 elements pointed to by xrange.
    pub yrange: *mut c_double, // The ranges of the bins in the y-direction are stored in an array of ny + 1 elements pointed to by yrange.
    pub bin: *mut c_double, /*The counts for each bin are stored in an array pointed to by bin. The bins are floating-point numbers, so you can increment them by non-integer values if necessary. The array bin stores the two dimensional array of bins in a single block of memory according to the mapping bin(i,j) = bin[i * ny + j].

The range for bin(i,j) is given by xrange[i] to xrange[i+1] in the x-direction and yrange[j] to yrange[j+1] in the y-direction. Each bin is inclusive at the lower end and exclusive at the upper end. Mathematically this means that the bins are defined by the following inequality,

bin(i,j) corresponds to xrange[i] <= x < xrange[i+1]
                    and yrange[j] <= y < yrange[j+1]
Note that any samples which fall on the upper sides of the histogram are excluded. If you want to include these values for the side bins you will need to add an extra row or column to your histogram.
*/
}

#[repr(C)]
pub struct gsl_histogram2d_pdf {
    pub nx: size_t, // This is the number of histogram bins used to approximate the probability distribution function in the x direction.
    pub ny: size_t, // This is the number of histogram bins used to approximate the probability distribution function in the y direction.
    pub xrange: *mut c_double, // The ranges of the bins in the x-direction are stored in an array of nx + 1 elements pointed to by xrange.
    pub yrange: *mut c_double, // The ranges of the bins in the y-direction are stored in an array of ny + 1 pointed to by yrange.
    pub sum: *mut c_double, // The cumulative probability for the bins is stored in an array of nx*ny elements pointed to by sum.
}
