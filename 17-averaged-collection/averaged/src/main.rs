use averaged::AveragedCollection;

mod lib;
fn main() {
    let mut collection = AveragedCollection {
        list: vec![1, 2, 3],
        average: 0.0,
    };

    collection.add(4);

    let result = collection.average();

    println!("Result: {}", result);
    println!("Hello, world!");
}
