//! Builds a [`WalkerTable`] instance.

use crate::table::WalkerTable;

pub trait NewBuilder<T> {
    /// Creates a new instance of [`WalkerTableBuilder`].
    fn new(index_weights: &Vec<T>) -> WalkerTableBuilder;
}

/// Builder of [`WalkerTable`]
///
/// ## Example
///
/// ```rust
/// use weighted_rand::builder::*;
///
/// fn main() {
///     let index_weights = vec![1, 2, 3, 4];
///     let builder = WalkerTableBuilder::new(&index_weights);
///     let wa_table = builder.build();
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

impl NewBuilder<u32> for WalkerTableBuilder {
    fn new(index_weights: &Vec<u32>) -> WalkerTableBuilder {
        let table_len = index_weights.len() as u32;

        // Process that the mean of index_weights does not become a float value
        let ws = index_weights
            .iter()
            .map(|w| w * table_len)
            .collect::<Vec<u32>>()
            .to_vec();

        WalkerTableBuilder { index_weights: ws }
    }
}

impl WalkerTableBuilder {
    /// Builds a new instance of [`WalkerTable`].
    pub fn build(&self) -> WalkerTable {
        let table_len = self.index_weights.len();

        if self.sum() == 0 {
            // Returns WalkerTable that performs unweighted random sampling.
            return WalkerTable::new(vec![0; table_len], vec![0; table_len], 1);
        }

        let (aliases, thresholds) = self.calc_table();

        WalkerTable::new(aliases, thresholds, self.mean())
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
    use crate::builder::*;
    use crate::table::WalkerTable;

    #[test]
    fn make_table_from_u32() {
        let index_weights = vec![2, 7, 9, 2, 4, 8, 1, 3, 6, 5];
        let builder = WalkerTableBuilder::new(&index_weights);
        let w_table = builder.build();

        let expected = WalkerTable::new(
            vec![2, 1, 1, 2, 2, 2, 5, 9, 5, 8],
            vec![27, 47, 23, 27, 7, 5, 37, 17, 1, 14],
            47,
        );

        assert_eq!(w_table, expected)
    }

    #[test]
    fn make_table_from_f32() {
        let index_weights = vec![0.1, 0.2, 0.3, 0.4];
        let builder = WalkerTableBuilder::new(&index_weights);
        let w_table = builder.build();

        let expected = WalkerTable::new(vec![3, 3, 2, 2], vec![6, 2, 10, 2], 10);

        assert_eq!(w_table, expected)
    }

    #[test]
    fn when_sum_is_zero() {
        let index_weights = vec![0; 5];
        let builder = WalkerTableBuilder::new(&index_weights);
        let w_table = builder.build();

        let expected = WalkerTable::new(vec![0; 5], vec![0; 5], 1);

        assert_eq!(w_table, expected)
    }
}
