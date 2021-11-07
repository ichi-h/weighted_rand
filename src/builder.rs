//! Builds a [`WalkerTable`] instance.

use crate::error::WeightedRandError;
use crate::table::WalkerTable;
use anyhow::Result;

/// Builder of [`WalkerTable`]
///
/// ## Example
///
/// ```rust
/// use weighted_rand::builder::WalkerTableBuilder;
///
/// fn main() {
///     let index_weights = vec![1, 2, 3, 4];
///     let mut builder = WalkerTableBuilder::new(index_weights);
///     let wa_table = builder.build().unwrap();
/// }
/// ```
///
/// ## About `index_weights`
///
/// `index_weights` is the weights of the output indexes.
///
/// The larger the value, the more likely the corresponding index will be
/// output.
///
/// For example, if this value is `vec![2, 1, 7, 0]`, the output probabilities
/// for each index are 0.2, 0.1, 0.7 and 0. If a weight value is 0, the
/// corresponding index will not be output. In other words, the index 3 will
/// not be output in the this cases.
///
/// The reason why the type is [`u32`] is because it is non-negative, and
/// can counter arithmetic error of floating point.
pub struct WalkerTableBuilder {
    /// Weights of the output indexes.
    index_weights: Vec<u32>,
}

impl WalkerTableBuilder {
    /// Creates a new instance of [`WalkerTableBuilder`].
    pub fn new(index_weights: Vec<u32>) -> WalkerTableBuilder {
        WalkerTableBuilder {
            index_weights: index_weights,
        }
    }

    /// Builds a new instance of [`WalkerTable`].
    pub fn build(&mut self) -> Result<WalkerTable> {
        if self.sum() == 0 {
            return Err(WeightedRandError::SumWeights)?;
        }

        let table_len = self.index_weights.len();

        self.index_weights = self
            .index_weights
            .iter()
            .map(|w| w * self.sum() * table_len as u32)
            .collect::<Vec<u32>>()
            .to_vec();

        let (aliases, thresholds) = self.calc_table();

        Ok(WalkerTable::new(aliases, thresholds, self.mean()))
    }

    /// Calculates the sum of `index_weights`.
    fn sum(&self) -> u32 {
        self.index_weights.iter().fold(0, |acc, cur| acc + cur)
    }

    /// Calculates the mean of `index_weights`.
    fn mean(&self) -> u32 {
        self.sum() / self.index_weights.len() as u32
    }

    /// Returns the tables of aliases and thresholds.
    fn calc_table(&self) -> (Vec<u32>, Vec<u32>) {
        let table_len = self.index_weights.len();
        let (mut below_vec, mut above_vec) = self.separate_weight();
        let mean = self.mean();

        let mut aliases = vec![0; table_len];
        let mut thresholds = vec![0; table_len];
        loop {
            match below_vec.pop() {
                Some(below) => {
                    if let Some(above) = above_vec.pop() {
                        let diff = mean - below.1;
                        aliases[below.0] = above.0 as u32;
                        thresholds[below.0] = diff;
                        if above.1 - diff <= mean {
                            below_vec.push((above.0, above.1 - diff));
                        } else {
                            above_vec.push((above.0, above.1 - diff));
                        }
                    } else {
                        aliases[below.0] = below.0 as u32;
                        thresholds[below.0] = below.1;
                    }
                }
                None => break,
            }
        }

        (aliases, thresholds)
    }

    /// Divide the values of `index_weights` based on the mean of them.
    ///
    /// The tail value is a weight and head is its index.
    fn separate_weight(&self) -> (Vec<(usize, u32)>, Vec<(usize, u32)>) {
        let mut below_vec = Vec::new();
        let mut above_vec = Vec::new();
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
    use crate::builder::WalkerTableBuilder;
    use crate::table::WalkerTable;

    #[test]
    fn make_table() {
        let index_weights = vec![2, 7, 9, 2, 4, 8, 1, 3, 6, 5];
        let mut builder = WalkerTableBuilder::new(index_weights);
        let w_table = builder.build().unwrap();

        let expected = WalkerTable::new(
            vec![2, 1, 1, 2, 2, 2, 5, 9, 5, 8],
            vec![1269, 2209, 1081, 1269, 329, 235, 1739, 799, 47, 658],
            2209,
        );

        assert_eq!(w_table, expected)
    }

    #[test]
    fn sum_error() {
        let index_weights = vec![0];
        let mut builder = WalkerTableBuilder::new(index_weights);

        match builder.build() {
            Err(_) => {}
            Ok(_) => panic!("'index_weights' with a total of 0 has passed the test."),
        }
    }
}
