//! Weighted random index generator by Walker's Alias Method.

use rand::prelude::*;
use serde::{Deserialize, Serialize};

/// Table of aliases and probabilities
///
/// In Walker's Alias Method, weighted random sampling is performed by the
/// following operations.
///
/// 1. Get an index "i" randomly.
/// 2. Get a random value "r" between 0 and 1
/// 3. If "r" exceeds the value of `probs[i]`, "i" is output as-is. If it
///    does not, the value of `aliases[i]` (which means the alias to another
///    index) is output.
///
/// The more likely a particular index is to be output, the more of the value
/// will be included in `aliases`.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct WalkerTable {
    /// Alias to another index
    aliases: Vec<usize>,

    /// Probability for whether to output the index attached to `aliases`.
    probs: Vec<f32>,
}

impl WalkerTable {
    /// Creates a new instance of [`WalkerTable`].
    pub fn new(aliases: Vec<usize>, probs: Vec<f32>) -> WalkerTable {
        WalkerTable {
            aliases: aliases,
            probs: probs,
        }
    }

    /// Returns an index at random.
    pub fn next(&self) -> usize {
        let mut rng = rand::thread_rng();
        self.next_rng(&mut rng)
    }

    /// Returns an index at random using an external RNG which implements Rng.
    pub fn next_rng(&self, rng: &mut impl Rng) -> usize {
        let i = rng.gen_range(0..self.probs.len());
        let r = rng.gen::<f32>();
        if r < self.probs[i] {
            return self.aliases[i];
        }
        i
    }
}

#[cfg(test)]
mod table_test {
    use crate::builder::*;
    use rand;

    const N: usize = 100_000;

    fn count<T: PartialEq>(slice: &[T], target: T) -> f32 {
        slice
            .iter()
            .fold(0.0, |acc, cur| if *cur == target { acc + 1.0 } else { acc })
    }

    #[test]
    fn unweighted_random_sampling() {
        let index_weights = [0; 4];
        let builder = WalkerTableBuilder::new(&index_weights);
        let wa_table = builder.build();

        let mut rng = rand::thread_rng();

        let idxs = (0..N)
            .map(|_| wa_table.next_rng(&mut rng))
            .collect::<Vec<usize>>();

        let i_0 = count(&idxs, 0);
        let i_1 = count(&idxs, 1);
        let i_2 = count(&idxs, 2);
        let i_3 = count(&idxs, 3);

        const EXPT: f32 = N as f32 * 0.25;
        assert!(
            (EXPT * 0.95 < i_0 && i_0 < EXPT * 1.05)
                && (EXPT * 0.95 < i_1 && i_1 < EXPT * 1.05)
                && (EXPT * 0.95 < i_2 && i_2 < EXPT * 1.05)
                && (EXPT * 0.95 < i_3 && i_3 < EXPT * 1.05)
        )
    }

    #[test]
    fn weighted_random_sampling() {
        let index_weights = [2, 1, 7, 0];
        let builder = WalkerTableBuilder::new(&index_weights);
        let wa_table = builder.build();

        let idxs = (0..N).map(|_| wa_table.next()).collect::<Vec<usize>>();

        let i_0 = count(&idxs, 0);
        let i_1 = count(&idxs, 1);
        let i_2 = count(&idxs, 2);
        let i_3 = count(&idxs, 3);

        const EXPT: [f32; 4] = [N as f32 * 0.2, N as f32 * 0.1, N as f32 * 0.7, 0.0];
        assert!(
            (EXPT[0] * 0.95 < i_0 && i_0 < EXPT[0] * 1.05)
                && (EXPT[1] * 0.95 < i_1 && i_1 < EXPT[1] * 1.05)
                && (EXPT[2] * 0.95 < i_2 && i_2 < EXPT[2] * 1.05)
                && (EXPT[3] == i_3)
        )
    }
}
