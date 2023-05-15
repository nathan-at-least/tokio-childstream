use crate::IntervalStream;
use tokio::time::{Duration, Interval};

pub trait IntoIntervalStream {
    fn into_interval_stream(self) -> IntervalStream;
}

impl IntoIntervalStream for Interval {
    fn into_interval_stream(self) -> IntervalStream {
        IntervalStream(self)
    }
}

impl IntoIntervalStream for Duration {
    fn into_interval_stream(self) -> IntervalStream {
        tokio::time::interval(self).into_interval_stream()
    }
}
