use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
    pub num_of_cores: usize,
    pub thread_count_per_core: usize,
}

impl Default for Cache {
    fn default() -> Self {
        let num_of_cores = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
        Self {
            num_of_cores,
            thread_count_per_core: num_of_cores,
        }
    }
}

impl Cache {
    pub fn new(number_of_cores: usize, number_of_threads_per_core: usize) -> Self {
        Self {
            num_of_cores: number_of_cores,
            thread_count_per_core: number_of_threads_per_core,
        }
    }

    pub fn total_thread(&self) -> usize {
        self.num_of_cores * self.thread_count_per_core
    }
}
