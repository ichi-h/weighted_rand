# weighted_rand

[![weighted_rand](https://github.com/ichi-h/weighted_rand/actions/workflows/weighted_rand.yml/badge.svg)](https://github.com/ichi-h/weighted_rand/actions/workflows/weighted_rand.yml)
[![Crates.io](https://img.shields.io/crates/v/weighted_rand)](https://crates.io/crates/weighted_rand)
[![docs.rs](https://img.shields.io/docsrs/weighted_rand)](https://docs.rs/weighted_rand)
[![Crates.io](https://img.shields.io/crates/l/weighted_rand)](LICENSE-APACHE)

A weighted random sampling crate using Walker's Alias Method.

Walker's Alias Method (WAM) is one method for performing weighted random sampling.  
WAM weights each index of a array by giving two pieces of information: an alias to a different index and a probability to decide whether to jump to that index.

WAM is a very fast algorithm, and its computational complexity of the search is O(1).  
The difference in complexity between WAM and the Cumulative Sum Method is as follows.

|       Algorithm       | Building table |  Search  |
| :-------------------: | :------------: | :------: |
| Walker's Alias Method |      O(N)      |   O(1)   |
| Cumulative Sum Method |      O(N)      | O(log N) |

The API documentation is [here](https://docs.rs/weighted_rand).

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
weighted_rand = "0.4"
```

## Example

```rust
use weighted_rand::builder::WalkerTableBuilder;

fn main() {
    let fruit = ["Apple", "Banana", "Orange", "Peach"];

    // Define the weights for each index corresponding
    // to the above list.
    // In the following case, the ratio of each weight
    // is "2 : 1 : 7 : 0", and the output probabilities
    // for each index are 0.2, 0.1, 0.7 and 0.
    let index_weights = [2, 1, 7, 0];

    let builder = WalkerTableBuilder::new(&index_weights);
    let wa_table = builder.build();

    for i in (0..10).map(|_| wa_table.next()) {
        println!("{}", fruit[i]);
    }
}
```

Also, `index_weiaghts` supports `&[f32]`, like:

```rust
use rand;
use weighted_rand::builder::*;

fn main() {
    // Coin with a 5% higher probability of heads than tails
    let cheating_coin = ["Heads!", "Tails!"];
    let index_weights = [0.55, 0.45];

    let builder = WalkerTableBuilder::new(&index_weights);
    let wa_table = builder.build();

    // If you want to process something in a large number of
    // loops, we recommend using the next_rng method with an
    // external ThreadRng instance.
    let mut result = [""; 10000];
    let mut rng = rand::thread_rng();
    for r in &mut result {
        let j = wa_table.next_rng(&mut rng);
        *r = cheating_coin[j];
    }

    // println!("{:?}", result);
}
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
