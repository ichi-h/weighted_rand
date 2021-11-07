//! # weighted_rand
//!
//! A weighted random sampling crate using Walker's Alias Method.
//!
//! ## Example
//!
//! ```rust
//! use weighted_rand::builder::WalkerTableBuilder;
//!
//! fn main() {
//!     let fruit = ["Apple", "Banana", "Orange", "Peach"];
//!
//!     // The weights of the output indexes.
//!     // The higher the weight, the more likely the corresponding index will be
//!     // output.
//!     // In the following case, the output probabilities for each index are 0.2,
//!     // 0.1, 0.7 and 0. If a weight value is 0, the corresponding index will not
//!     // be output. In other words, the index 3 will not be output in this case.
//!     let index_weights = vec![2, 1, 7, 0];
//!
//!     let mut builder = WalkerTableBuilder::new(index_weights);
//!     let wa_table = builder.build().unwrap();
//!
//!     for _ in 0..10 {
//!         let i = wa_table.next(); // Will output 0, 1, or 2
//!         println!("{}", fruit[i]);
//!     }
//! }
//! ```
//!

pub mod builder;
pub mod error;
pub mod table;
