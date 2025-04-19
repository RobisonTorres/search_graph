use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::HashSet;
use serde_json::Result;

#[derive(serde::Deserialize)]
struct Product {
    price: f64,
    brand: String,
    category: String,
    recommendation: HashSet<String>,
}

fn main() -> Result<()> {

    let file = File::open("products.json").expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut product_list: HashMap<String, Product> = serde_json::from_reader(reader)?; 

    let item = String::from("Smartphone");
    product_list.get_mut(&item).unwrap().recommendation.insert("Mouse".to_string());

    let input = "Water".to_string();
    if let Some(info) = product_list.get(&input) {
        println!("{} - ${:.2} - {}.", input, info.price, info.brand);
        println!("Products relates");
        for p in info.recommendation.iter() {
            print!("{p} ")
        }
    } else {
        println!("Product related to your search {}.", input);
        for p in product_list.keys() {
            let product = product_list.get(p);
            if input == product.unwrap().brand || input == product.unwrap().category { 
                println!("{}", p);
            }
        }
    }
    Ok(())
}