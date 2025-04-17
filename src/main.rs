use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use serde_json::Result;

fn main() -> Result<()> {
    let file = File::open("products.json").expect("Failed to open file");
    let reader = BufReader::new(file);

    type ProductDetails = (f64, String);
    type ProductMap = HashMap<String, ProductDetails>;
    type CategoryMap = HashMap<String, ProductMap>;

    let products: Vec<CategoryMap> = serde_json::from_reader(reader)?;

    for category_map in products {
        for (category, items) in category_map {
            println!("Category: {}", category);
            for (product, (price, brand)) in items {
                println!("  - {}: ${:.2} ({})", product, price, brand);
            }
        }
    }

    Ok(())
}