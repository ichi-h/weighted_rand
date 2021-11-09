//! # weighted_rand
//!
//! A weighted random sampling crate using Walker's Alias Method.
//!
//! Walker's Alias Method (WAM) is one method for performing weighted
//! random sampling. WAM weights each index of a array by giving two
//! pieces of information: an alias to a different index and a threshold
//! (or a probability) to decide whether to jump to that index.
//!
//! ## Example
//!
//! ```rust
//! use weighted_rand::builder::*;
//!
//! fn main() {
//!     let fruit = ["Apple", "Banana", "Orange", "Peach"];
//!
//!     // Define the weights for each index corresponding
//!     // to the above list.
//!     // In the following case, the ratio of each weight
//!     // is "2 : 1 : 7 : 0", and the output probabilities
//!     // for each index are 0.2, 0.1, 0.7 and 0.
//!     let index_weights = vec![2, 1, 7, 0];
//!
//!     let builder = WalkerTableBuilder::new(&index_weights);
//!     let wa_table = builder.build();
//!
//!     for _ in 0..10 {
//!         let i = wa_table.next(); // Will output 0, 1, or 2
//!         println!("{}", fruit[i]);
//!     }
//! }
//! ```
//!

pub mod builder;
pub mod table;

mod util;
