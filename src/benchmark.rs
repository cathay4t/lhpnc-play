// SPDX-License-Identifier: Apache-2.0

use std::time::Instant;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct LhpncBenchmark {
    pub(crate) name: String,
    #[serde(skip)]
    start_time: Instant,
    /// User facing time for finishing the massive hello task
    pub(crate) total_ns: u128,
    /// User facing time for each iteration
    pub(crate) per_ns: u128,
}

impl LhpncBenchmark {
    pub(crate) fn start(name: &str) -> Self {
        Self {
            name: name.to_string(),
            start_time: Instant::now(),
            total_ns: 0,
            per_ns: 0,
        }
    }

    pub(crate) fn end(&mut self, iteration_count: u128) {
        self.total_ns = self.start_time.elapsed().as_nanos();
        self.per_ns = self.total_ns / iteration_count;
    }
}
