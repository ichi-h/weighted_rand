//! Builds a [`WalkerTable`] instance.

use crate::table::WalkerTable;
use crate::util::math::gcd_for_slice;

pub trait NewBuilder<T> {
    /// Creates a new instance of [`WalkerTableBuilder`] from
    /// [`&[u32]`] or [`&[f32]`].
    ///
    /// Values less than 0 will be calculated as 0.
    fn new(index_weights: &[T]) -> WalkerTableBuilder;
}

/// Builder of [`WalkerTable`]
///
/// ## Example
///
/// ```rust
/// use weighted_rand::builder::*;
///
/// fn main() {
///     let index_weights = [1, 2, 3, 4];
///     let builder = WalkerTableBuilder::new(&index_weights);
///     let wa_table = builder.build();
/// }
/// ```
///
/// Also, `index_weiaghts` supports [`&[f32]`], like `[0.1, 0.2, 0.3, 0.4]`
///
/// ## About `index_weights`
///
/// `index_weights` is the weights of the output indexes.
///
/// The larger the value, the more likely the corresponding index will be
/// output.
///
/// For example, if this value is `[2, 1, 7, 0]`, the output probabilities
/// for each index are 0.2, 0.1, 0.7 and 0. If a weight value is 0, the
/// corresponding index will not be output. In other words, the index 3 will
/// not be output in the this cases.
pub struct WalkerTableBuilder {
    /// Weights of the output indexes.
    index_weights: Vec<u32>,
}

impl NewBuilder<u32> for WalkerTableBuilder {
    fn new(index_weights: &[u32]) -> WalkerTableBuilder {
        let table_len = index_weights.len() as u32;

        // Process that the mean of index_weights does not become a float value
        let ws = index_weights
            .iter()
            .map(|w| w * table_len)
            .collect::<Vec<u32>>();

        WalkerTableBuilder { index_weights: ws }
    }
}

impl NewBuilder<f32> for WalkerTableBuilder {
    fn new(index_weights: &[f32]) -> WalkerTableBuilder {
        let ws = index_weights
            .iter()
            .map(|w| (w * 10000.0).round() as u32)
            .collect::<Vec<u32>>();

        let gcd = gcd_for_slice(&ws);
        let ws = ws.iter().map(|w| w / gcd).collect::<Vec<u32>>();

        WalkerTableBuilder::new(&ws)
    }
}

impl WalkerTableBuilder {
    /// Builds a new instance of [`WalkerTable`].
    pub fn build(&self) -> WalkerTable {
        let table_len = self.index_weights.len();

        if self.sum() == 0 {
            // Returns WalkerTable that performs unweighted random sampling.
            return WalkerTable::new(vec![0; table_len], vec![0.0; table_len]);
        }

        let (aliases, probs) = self.calc_table();

        WalkerTable::new(aliases, probs)
    }

    /// Inverses given weights
    pub fn inverse(self) -> WalkerTableBuilder {
        let min_value = match self.index_weights.iter().min() {
            Some(v) => *v,
            None => 0,
        };
        let max_value = match self.index_weights.iter().max() {
            Some(v) => *v,
            None => 0,
        };
        Self {
            index_weights: self
                .index_weights
                .into_iter()
                .map(|x| {
                    if x == max_value {
                        min_value
                    } else if x == min_value {
                        max_value
                    } else {
                        max_value - x
                    }
                })
                .collect(),
        }
    }

    /// Calculates the sum of `index_weights`.
    fn sum(&self) -> u32 {
        self.index_weights.iter().fold(0, |acc, cur| acc + cur)
    }

    /// Calculates the mean of `index_weights`.
    fn mean(&self) -> u32 {
        self.sum() / self.index_weights.len() as u32
    }

    /// Returns the tables of aliases and probabilities.
    fn calc_table(&self) -> (Vec<usize>, Vec<f32>) {
        let table_len = self.index_weights.len();
        let (mut below_vec, mut above_vec) = self.separate_weight();
        let mean = self.mean();

        let mut aliases = vec![0; table_len];
        let mut probs = vec![0.0; table_len];
        loop {
            match below_vec.pop() {
                Some(below) => {
                    if let Some(above) = above_vec.pop() {
                        let diff = mean - below.1;
                        aliases[below.0] = above.0 as usize;
                        probs[below.0] = diff as f32 / mean as f32;
                        if above.1 - diff <= mean {
                            below_vec.push((above.0, above.1 - diff));
                        } else {
                            above_vec.push((above.0, above.1 - diff));
                        }
                    } else {
                        aliases[below.0] = below.0 as usize;
                        probs[below.0] = below.1 as f32 / mean as f32;
                    }
                }
                None => break,
            }
        }

        (aliases, probs)
    }

    /// Divide the values of `index_weights` based on the mean of them.
    ///
    /// The tail value is a weight and head is its index.
    fn separate_weight(&self) -> (Vec<(usize, u32)>, Vec<(usize, u32)>) {
        let mut below_vec = Vec::with_capacity(self.index_weights.len());
        let mut above_vec = Vec::with_capacity(self.index_weights.len());
        for (i, w) in self.index_weights.iter().enumerate() {
            if *w <= self.mean() {
                below_vec.push((i, *w));
            } else {
                above_vec.push((i, *w));
            }
        }
        (below_vec, above_vec)
    }
}

#[cfg(test)]
mod builder_test {
    use crate::builder::*;
    use crate::table::WalkerTable;

    #[test]
    fn make_table_from_u32() {
        let index_weights = [2, 7, 9, 2, 4, 8, 1, 3, 6, 5];
        let builder = WalkerTableBuilder::new(&index_weights);
        let w_table = builder.build();

        let expected = WalkerTable::new(
            vec![2, 1, 1, 2, 2, 2, 5, 9, 5, 8],
            vec![
                0.574468085106383,
                1.0,
                0.48936170212766,
                0.574468085106383,
                0.148936170212766,
                0.106382978723404,
                0.787234042553192,
                0.361702127659574,
                0.0212765957446809,
                0.297872340425532,
            ],
        );

        assert_eq!(w_table, expected)
    }

    #[test]
    fn make_table_from_f32() {
        let index_weights = [0.1, 0.2, 0.3, -0.4];
        let builder = WalkerTableBuilder::new(&index_weights);
        let w_table = builder.build();

        let expected = WalkerTable::new(vec![1, 1, 1, 2], vec![0.333333333333333, 1.0, 0.0, 1.0]);

        assert_eq!(w_table, expected)
    }

    #[test]
    fn when_sum_is_zero() {
        let index_weights = [0; 5];
        let builder = WalkerTableBuilder::new(&index_weights);
        let w_table = builder.build();

        let expected = WalkerTable::new(vec![0; 5], vec![0.0; 5]);

        assert_eq!(w_table, expected)
    }
}
