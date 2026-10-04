mod models;
use models::Product;
fn main() {
    let p = Product {
        name: "Widget".to_string(),
        price: 9.99,
    };

    println!("{} costs {}", p.name, p.price);
}
