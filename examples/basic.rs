use weighted_rand::builder::WalkerTableBuilder;

fn main() {
    let fruit = ["Apple", "Banana", "Orange", "Peach"];

    // Define the weights for each index corresponding
    // to the above list.
    // In the following case, the ratio of each weight
    // is "2 : 1 : 7 : 0", and the output probabilities
    // for each index are 0.2, 0.1, 0.7 and 0.
    let index_weights = vec![2, 1, 7, 0];

    let mut builder = WalkerTableBuilder::new(index_weights);
    let wa_table = builder.build();

    for _ in 0..10 {
        let i = wa_table.next(); // Will output 0, 1, or 2
        println!("{}", fruit[i]);
    }
}
