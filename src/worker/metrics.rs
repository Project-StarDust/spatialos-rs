use spatialos_sys::*;

use std::ffi::CStr;

#[derive(Debug)]
pub struct HistogramMetricBucket {
    pub upper_bound: f64,
    pub samples: u32,
}

impl From<Worker_HistogramMetricBucket> for HistogramMetricBucket {
    fn from(histogram_bucket: Worker_HistogramMetricBucket) -> Self {
        Self {
            upper_bound: histogram_bucket.upper_bound,
            samples: histogram_bucket.samples,
        }
    }
}

#[derive(Debug)]
pub struct HistogramMetric {
    pub key: String,
    pub sum: f64,
    pub bucket_count: u32,
    pub buckets: Vec<HistogramMetricBucket>,
}

impl From<Worker_HistogramMetric> for HistogramMetric {
    fn from(metric: Worker_HistogramMetric) -> Self {
        let key = unsafe { CStr::from_ptr(metric.key) }
            .to_str()
            .map(|s| s.to_owned())
            .unwrap();

        let buckets = unsafe {
            let mut buckets = Vec::new();
            for index in 0..metric.bucket_count {
                let bucket_ptr = metric.buckets.offset(index as isize as isize);
                buckets.push(HistogramMetricBucket::from(*bucket_ptr))
            }
            buckets
        };
        Self {
            sum: metric.sum,
            bucket_count: metric.bucket_count,
            key,
            buckets,
        }
    }
}

#[derive(Debug)]
#[doc = " Parameters for a gauge metric."]
pub struct GaugeMetric {
    pub key: String,
    pub value: f64,
}

impl From<Worker_GaugeMetric> for GaugeMetric {
    fn from(metric: Worker_GaugeMetric) -> Self {
        let key = unsafe { CStr::from_ptr(metric.key) }
            .to_str()
            .map(|s| s.to_owned())
            .unwrap();
        Self {
            value: metric.value,
            key,
        }
    }
}

#[derive(Debug)]
#[doc = " Parameters for sending metrics to SpatialOS."]
pub struct Metrics {
    #[doc = " The load value of this worker. If NULL, do not report load."]
    pub load: Option<f64>,
    #[doc = " The number of gauge metrics."]
    pub gauge_metric_count: u32,
    #[doc = " Array of gauge metrics."]
    pub gauge_metrics: Vec<GaugeMetric>,
    #[doc = " The number of histogram metrics."]
    pub histogram_metric_count: u32,
    #[doc = " Array of histogram metrics."]
    pub histogram_metrics: Vec<HistogramMetric>,
}

impl From<Worker_Metrics> for Metrics {
    fn from(metrics: Worker_Metrics) -> Self {
        let histogram_metrics = unsafe {
            let mut histogram_metrics = Vec::new();
            for index in 0..metrics.histogram_metric_count {
                let histogram_metric_ptr =
                    metrics.histogram_metrics.offset(index as isize as isize);
                histogram_metrics.push(HistogramMetric::from(*histogram_metric_ptr))
            }
            histogram_metrics
        };
        let gauge_metrics = unsafe {
            let mut gauge_metrics = Vec::new();
            for index in 0..metrics.gauge_metric_count {
                let gauge_metric_ptr = metrics.gauge_metrics.offset(index as isize as isize);
                gauge_metrics.push(GaugeMetric::from(*gauge_metric_ptr))
            }
            gauge_metrics
        };
        if metrics.load.is_null() {
            Self {
                load: None,
                gauge_metric_count: metrics.gauge_metric_count,
                histogram_metric_count: metrics.histogram_metric_count,
                histogram_metrics,
                gauge_metrics,
            }
        } else {
            Self {
                load: Some(unsafe { *metrics.load }),
                gauge_metric_count: metrics.gauge_metric_count,
                histogram_metric_count: metrics.histogram_metric_count,
                histogram_metrics,
                gauge_metrics,
            }
        }
    }
}
