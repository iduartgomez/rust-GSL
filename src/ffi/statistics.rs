//! Statistics

use libc::{c_double, size_t};

extern "C" {
    // Mean, Standard Deviation and Variance
    pub fn gsl_stats_mean(data: *const c_double, stride: size_t, n: size_t) -> c_double;
    pub fn gsl_stats_variance(data: *const c_double, stride: size_t, n: size_t) -> c_double;
    pub fn gsl_stats_variance_m(data: *const c_double,
                                stride: size_t,
                                n: size_t,
                                mean: c_double)
                                -> c_double;
    pub fn gsl_stats_sd(data: *const c_double, stride: size_t, n: size_t) -> c_double;
    pub fn gsl_stats_sd_m(data: *const c_double,
                          stride: size_t,
                          n: size_t,
                          mean: c_double)
                          -> c_double;
    pub fn gsl_stats_tss(data: *const c_double, stride: size_t, n: size_t) -> c_double;
    pub fn gsl_stats_tss_m(data: *const c_double,
                           stride: size_t,
                           n: size_t,
                           mean: c_double)
                           -> c_double;
    pub fn gsl_stats_variance_with_fixed_mean(data: *const c_double,
                                              stride: size_t,
                                              n: size_t,
                                              mean: c_double)
                                              -> c_double;
    pub fn gsl_stats_sd_with_fixed_mean(data: *const c_double,
                                        stride: size_t,
                                        n: size_t,
                                        mean: c_double)
                                        -> c_double;
    // Absolute deviation
    pub fn gsl_stats_absdev(data: *const c_double, stride: size_t, n: size_t) -> c_double;
    pub fn gsl_stats_absdev_m(data: *const c_double,
                              stride: size_t,
                              n: size_t,
                              mean: c_double)
                              -> c_double;
    // Higher moments (skewness and kurtosis)
    pub fn gsl_stats_skew(data: *const c_double, stride: size_t, n: size_t) -> c_double;
    pub fn gsl_stats_skew_m_sd(data: *const c_double,
                               stride: size_t,
                               n: size_t,
                               mean: c_double,
                               sd: c_double)
                               -> c_double;
    pub fn gsl_stats_kurtosis(data: *const c_double, stride: size_t, n: size_t) -> c_double;
    pub fn gsl_stats_kurtosis_m_sd(data: *const c_double,
                                   stride: size_t,
                                   n: size_t,
                                   mean: c_double,
                                   sd: c_double)
                                   -> c_double;
    // Autocorrelation
    pub fn gsl_stats_lag1_autocorrelation(data: *const c_double,
                                          stride: size_t,
                                          n: size_t)
                                          -> c_double;
    pub fn gsl_stats_lag1_autocorrelation_m(data: *const c_double,
                                            stride: size_t,
                                            n: size_t,
                                            mean: c_double)
                                            -> c_double;
    // Covariance
    pub fn gsl_stats_covariance(data1: *const c_double,
                                stride1: size_t,
                                data2: *const c_double,
                                stride2: size_t,
                                n: size_t)
                                -> c_double;
    pub fn gsl_stats_covariance_m(data1: *const c_double,
                                  stride1: size_t,
                                  data2: *const c_double,
                                  stride2: size_t,
                                  n: size_t,
                                  mean1: c_double,
                                  mean2: c_double)
                                  -> c_double;
    // Correlation
    pub fn gsl_stats_correlation(data1: *const c_double,
                                 stride1: size_t,
                                 data2: *const c_double,
                                 stride2: size_t,
                                 n: size_t)
                                 -> c_double;
    pub fn gsl_stats_spearman(data1: *const c_double,
                              stride1: size_t,
                              data2: *const c_double,
                              stride2: size_t,
                              n: size_t,
                              work: *mut c_double)
                              -> c_double;
    // Weighted Samples
    pub fn gsl_stats_wmean(w: *const c_double,
                           wstride: size_t,
                           data: *const c_double,
                           stride: size_t,
                           n: size_t)
                           -> c_double;
    pub fn gsl_stats_wvariance(w: *const c_double,
                               wstride: size_t,
                               data: *const c_double,
                               stride: size_t,
                               n: size_t)
                               -> c_double;
    pub fn gsl_stats_wvariance_m(w: *const c_double,
                                 wstride: size_t,
                                 data: *const c_double,
                                 stride: size_t,
                                 n: size_t,
                                 wmean: c_double)
                                 -> c_double;
    pub fn gsl_stats_wsd(w: *const c_double,
                         wstride: size_t,
                         data: *const c_double,
                         stride: size_t,
                         n: size_t)
                         -> c_double;
    pub fn gsl_stats_wsd_m(w: *const c_double,
                           wstride: size_t,
                           data: *const c_double,
                           stride: size_t,
                           n: size_t,
                           wmean: c_double)
                           -> c_double;
    pub fn gsl_stats_wvariance_with_fixed_mean(w: *const c_double,
                                               wstride: size_t,
                                               data: *const c_double,
                                               stride: size_t,
                                               n: size_t,
                                               wmean: c_double)
                                               -> c_double;
    pub fn gsl_stats_wsd_with_fixed_mean(w: *const c_double,
                                         wstride: size_t,
                                         data: *const c_double,
                                         stride: size_t,
                                         n: size_t,
                                         wmean: c_double)
                                         -> c_double;
    pub fn gsl_stats_wtss(w: *const c_double,
                          wstride: size_t,
                          data: *const c_double,
                          stride: size_t,
                          n: size_t)
                          -> c_double;
    pub fn gsl_stats_wtss_m(w: *const c_double,
                            wstride: size_t,
                            data: *const c_double,
                            stride: size_t,
                            n: size_t,
                            wmean: c_double)
                            -> c_double;
    pub fn gsl_stats_wabsdev(w: *const c_double,
                             wstride: size_t,
                             data: *const c_double,
                             stride: size_t,
                             n: size_t)
                             -> c_double;
    pub fn gsl_stats_wabsdev_m(w: *const c_double,
                               wstride: size_t,
                               data: *const c_double,
                               stride: size_t,
                               n: size_t,
                               wmean: c_double)
                               -> c_double;
    pub fn gsl_stats_wskew(w: *const c_double,
                           wstride: size_t,
                           data: *const c_double,
                           stride: size_t,
                           n: size_t)
                           -> c_double;
    pub fn gsl_stats_wskew_m_sd(w: *const c_double,
                                wstride: size_t,
                                data: *const c_double,
                                stride: size_t,
                                n: size_t,
                                wmean: c_double,
                                wsd: c_double)
                                -> c_double;
    pub fn gsl_stats_wkurtosis(w: *const c_double,
                               wstride: size_t,
                               data: *const c_double,
                               stride: size_t,
                               n: size_t)
                               -> c_double;
    pub fn gsl_stats_wkurtosis_m_sd(w: *const c_double,
                                    wstride: size_t,
                                    data: *const c_double,
                                    stride: size_t,
                                    n: size_t,
                                    wmean: c_double,
                                    wsd: c_double)
                                    -> c_double;
    // Maximum and Minimum values
    pub fn gsl_stats_max(data: *const c_double, stride: size_t, n: size_t) -> c_double;
    pub fn gsl_stats_min(data: *const c_double, stride: size_t, n: size_t) -> c_double;
    pub fn gsl_stats_minmax(min: *mut c_double,
                            max: *mut c_double,
                            data: *const c_double,
                            stride: size_t,
                            n: size_t);
    pub fn gsl_stats_max_index(data: *const c_double, stride: size_t, n: size_t) -> size_t;
    pub fn gsl_stats_min_index(data: *const c_double, stride: size_t, n: size_t) -> size_t;
    pub fn gsl_stats_minmax_index(min: *mut size_t,
                                  max: *mut size_t,
                                  data: *const c_double,
                                  stride: size_t,
                                  n: size_t);
    // Median and Percentiles
    pub fn gsl_stats_median_from_sorted_data(data: *const c_double,
                                             stride: size_t,
                                             n: size_t)
                                             -> c_double;
    pub fn gsl_stats_quantile_from_sorted_data(data: *const c_double,
                                               stride: size_t,
                                               n: size_t,
                                               f: c_double)
                                               -> c_double;
}
