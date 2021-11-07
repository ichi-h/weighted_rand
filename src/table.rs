//! Weighted random index generator by Walker's Alias Method.

use rand::prelude::*;
use serde::{Deserialize, Serialize};

/// Table of aliases and thresholds
///
/// In Walker's Alias Method, weighted random sampling is performed by the
/// following operations.
///
/// 1. Get an index "i" randomly.
/// 2. Get a random value "r" between 0 and `max_thold`
/// 3. If "r" exceeds the value of `thresholds[i]`, "i" is output as-is. If it
///    does not, the value of `aliases[i]` (which means the alias to another
///    index) is output.
///
/// The more likely a particular index is to be output, the more of the value
/// will be included in `aliases`.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct WalkerTable {
    /// Alias to another index
    aliases: Vec<u32>,

    /// Threshold for whether to output the index attached to `aliases`.
    thresholds: Vec<u32>,

    /// Maximum threshold value
    pub max_thold: u32,
}

impl WalkerTable {
    /// Creates a new instance of [`WalkerTable`].
    pub fn new(aliases: Vec<u32>, thresholds: Vec<u32>, max_thold: u32) -> WalkerTable {
        WalkerTable {
            aliases: aliases,
            thresholds: thresholds,
            max_thold: max_thold,
        }
    }

    /// Returns a index at random.
    pub fn next(&self) -> usize {
        let mut rng = rand::thread_rng();
        let i = rng.gen::<usize>() % self.thresholds.len();
        let r = rng.gen_range(0..self.max_thold);
        if r < self.thresholds[i] {
            self.aliases[i] as usize
        } else {
            i
        }
    }
}

#[cfg(test)]
mod table_test {
    use crate::builder::WalkerTableBuilder;

    #[test]
    fn unweighted_random_sampling() {
        let index_weights = vec![1, 1, 1, 1];
        let mut builder = WalkerTableBuilder::new(index_weights);
        let wa_table = builder.build().unwrap();

        const N: usize = 100_000;
        const P: f32 = 0.25;
        const EXPT: f32 = N as f32 * P;

        let idxs = (0..N)
            .map(|_| wa_table.next())
            .collect::<Vec<usize>>()
            .to_vec();

        let i_0 = idxs
            .iter()
            .fold(0, |acc, cur| if *cur == 0 { acc + 1 } else { acc }) as f32;
        let i_1 = idxs
            .iter()
            .fold(0, |acc, cur| if *cur == 1 { acc + 1 } else { acc }) as f32;
        let i_2 = idxs
            .iter()
            .fold(0, |acc, cur| if *cur == 2 { acc + 1 } else { acc }) as f32;
        let i_3 = idxs
            .iter()
            .fold(0, |acc, cur| if *cur == 3 { acc + 1 } else { acc }) as f32;

        assert!(
            (EXPT * 0.95 < i_0 && i_0 < EXPT * 1.05)
                && (EXPT * 0.95 < i_1 && i_1 < EXPT * 1.05)
                && (EXPT * 0.95 < i_2 && i_2 < EXPT * 1.05)
                && (EXPT * 0.95 < i_3 && i_3 < EXPT * 1.05)
        )
    }

    #[test]
    fn weighted_random_sampling() {
        let index_weights = vec![2, 1, 7, 0];
        let mut builder = WalkerTableBuilder::new(index_weights);
        let wa_table = builder.build().unwrap();

        const N: usize = 100_000;
        const EXPT: [f32; 4] = [N as f32 * 0.2, N as f32 * 0.1, N as f32 * 0.7, 0.0];

        let idxs = (0..N)
            .map(|_| wa_table.next())
            .collect::<Vec<usize>>()
            .to_vec();

        let i_0 = idxs
            .iter()
            .fold(0, |acc, cur| if *cur == 0 { acc + 1 } else { acc }) as f32;
        let i_1 = idxs
            .iter()
            .fold(0, |acc, cur| if *cur == 1 { acc + 1 } else { acc }) as f32;
        let i_2 = idxs
            .iter()
            .fold(0, |acc, cur| if *cur == 2 { acc + 1 } else { acc }) as f32;
        let i_3 = idxs
            .iter()
            .fold(0, |acc, cur| if *cur == 3 { acc + 1 } else { acc }) as f32;

        assert!(
            (EXPT[0] * 0.95 < i_0 && i_0 < EXPT[0] * 1.05)
                && (EXPT[1] * 0.95 < i_1 && i_1 < EXPT[1] * 1.05)
                && (EXPT[2] * 0.95 < i_2 && i_2 < EXPT[2] * 1.05)
                && (EXPT[3] == i_3)
        )
    }
}
