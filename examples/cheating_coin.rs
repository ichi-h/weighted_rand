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
