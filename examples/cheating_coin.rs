use weighted_rand::builder::*;

fn main() {
    // Coins with a 5% higher probability of heads than tails
    let cheating_coin = ["Heads!", "Tails!"];
    let index_weights = vec![0.55, 0.45];

    let builder = WalkerTableBuilder::new(&index_weights);
    let wa_table = builder.build();

    for _ in 0..10 {
        let i = wa_table.next();
        println!("{}", cheating_coin[i]);
    }
}
