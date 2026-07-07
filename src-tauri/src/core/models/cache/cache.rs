use crate::core::constant::{TREE_INSERT_SEQ, TREE_INSERT_TOTAL};
use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering;

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

    pub fn reset_tree_build_progress(total_exptected: u64) {
        TREE_INSERT_SEQ.store(0, Ordering::Relaxed);
        TREE_INSERT_TOTAL.store(total_exptected, Ordering::Relaxed);
    }

    pub fn log_cache_tree_insert(path_str: &str, is_dir: bool) {
        let i = TREE_INSERT_SEQ.fetch_add(1, Ordering::Relaxed) + 1;
        let total = TREE_INSERT_TOTAL.load(Ordering::Relaxed);
        let kind = if is_dir { "DIR" } else { "FILE" };
        if total > 0 {
            println!("[cache-tree] {i:>7}/{total:<7} {kind} {path_str}");
        } else {
            println!("[cache-tree] {i:>7} {kind} {path_str}");
        }
    }
}
