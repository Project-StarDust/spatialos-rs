use spatialos_sys::{
    Worker_GaugeMetric, Worker_HistogramMetric, Worker_HistogramMetricBucket, Worker_Metrics,
};

use std::ffi::CString;

use crate::{const_to_string, const_to_vector, vector_to_owned_array};

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

impl Into<Worker_HistogramMetricBucket> for HistogramMetricBucket {
    fn into(self) -> Worker_HistogramMetricBucket {
        Worker_HistogramMetricBucket {
            upper_bound: self.upper_bound,
            samples: self.samples,
        }
    }
}

#[derive(Debug)]
pub struct HistogramMetric {
    pub key: String,
    pub sum: f64,
    pub buckets: Vec<HistogramMetricBucket>,
}

impl From<Worker_HistogramMetric> for HistogramMetric {
    fn from(metric: Worker_HistogramMetric) -> Self {
        let key = const_to_string(metric.key);
        let buckets = const_to_vector::<Worker_HistogramMetricBucket>(
            metric.buckets,
            metric.bucket_count as isize,
        );
        Self {
            sum: metric.sum,
            buckets: buckets.into_iter().map(|h| h.into()).collect(),
            key,
        }
    }
}

impl Into<Worker_HistogramMetric> for HistogramMetric {
    fn into(self) -> Worker_HistogramMetric {
        let key = CString::new(self.key).unwrap().into_raw();
        let buckets = self
            .buckets
            .into_iter()
            .map(|b| b.into())
            .collect::<Vec<_>>();

        let (buckets, bucket_count) = vector_to_owned_array(buckets);
        Worker_HistogramMetric {
            sum: self.sum,
            bucket_count: bucket_count as u32,
            key,
            buckets,
        }
    }
}

#[derive(Debug)]
/// Parameters for a gauge metric.
pub struct GaugeMetric {
    pub key: String,
    pub value: f64,
}

impl From<Worker_GaugeMetric> for GaugeMetric {
    fn from(metric: Worker_GaugeMetric) -> Self {
        let key = const_to_string(metric.key);
        Self {
            value: metric.value,
            key,
        }
    }
}

impl Into<Worker_GaugeMetric> for GaugeMetric {
    fn into(self) -> Worker_GaugeMetric {
        let key = CString::new(self.key).unwrap().into_raw();
        Worker_GaugeMetric {
            value: self.value,
            key,
        }
    }
}

#[derive(Debug)]
/// Parameters for sending metrics to SpatialOS.
pub struct Metrics {
    /// The load value of this worker. If NULL, do not report load.
    pub load: Option<f64>,
    /// Array of gauge metrics.
    pub gauge_metrics: Vec<GaugeMetric>,
    /// Array of histogram metrics.
    pub histogram_metrics: Vec<HistogramMetric>,
}

impl From<Worker_Metrics> for Metrics {
    fn from(metrics: Worker_Metrics) -> Self {
        let histogram_metrics = const_to_vector::<Worker_HistogramMetric>(
            metrics.histogram_metrics,
            metrics.histogram_metric_count as isize,
        );
        let gauge_metrics = const_to_vector::<Worker_GaugeMetric>(
            metrics.gauge_metrics,
            metrics.gauge_metric_count as isize,
        );
        if metrics.load.is_null() {
            Self {
                load: None,
                histogram_metrics: histogram_metrics.into_iter().map(|h| h.into()).collect(),
                gauge_metrics: gauge_metrics.into_iter().map(|h| h.into()).collect(),
            }
        } else {
            Self {
                load: Some(unsafe { *metrics.load }),
                histogram_metrics: histogram_metrics.into_iter().map(|h| h.into()).collect(),
                gauge_metrics: gauge_metrics.into_iter().map(|h| h.into()).collect(),
            }
        }
    }
}

impl Into<Worker_Metrics> for Metrics {
    fn into(self) -> Worker_Metrics {
        let histogram_metrics = self
            .histogram_metrics
            .into_iter()
            .map(|h| h.into())
            .collect::<Vec<_>>();
        let gauge_metrics = self
            .gauge_metrics
            .into_iter()
            .map(|g| g.into())
            .collect::<Vec<_>>();
        let (histogram_metrics, histogram_metric_count) = vector_to_owned_array(histogram_metrics);
        let (gauge_metrics, gauge_metric_count) = vector_to_owned_array(gauge_metrics);
        Worker_Metrics {
            load: if let Some(load) = self.load {
                &load as *const f64
            } else {
                std::ptr::null()
            },
            gauge_metric_count: gauge_metric_count as u32,
            histogram_metric_count: histogram_metric_count as u32,
            histogram_metrics,
            gauge_metrics,
        }
    }
}
