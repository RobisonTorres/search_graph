use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use serde_json::Result;

#[derive(serde::Deserialize)]
struct Product {
    price: f64,
    brand: String,
    recommendation: Vec<String>,
}

fn main() -> Result<()> {

    let file = File::open("products.json").expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut product_list: HashMap<String, Product> = serde_json::from_reader(reader)?; 

    let item = String::from("Smartphone");
    product_list.get_mut(&item).unwrap().recommendation.push("Mouse".to_string());
    if let Some(info) = product_list.get(&item) {
        println!("{} - ${:.2} - {}.", item, info.price, info.brand);
        println!("Products relates");
        for p in info.recommendation.iter() {
            print!("{p} ")
        }
    } else {
        println!("Item '{}' not found.", item);
    }
    Ok(())
}