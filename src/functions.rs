use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(serde::Deserialize)]
pub struct Product {
    pub price: f64,
    pub brand: String,
    pub category: String,
    pub recommendation: HashSet<String>,
}

pub fn read_json() -> HashMap<String, Product> {

    // This function...
    let file = File::open("products.json").expect("Failed to open file");
    let reader = BufReader::new(file);  
    let product_list: HashMap<String, Product> = serde_json::from_reader(reader).expect("REASON"); 
    product_list
}

pub fn search_product_by_name(input_user: String) -> bool {

    // This function...
    let product_list = read_json();
    if let Some(product) = product_list.get(&input_user) {
        println!("{} - ${:.2} - {}.", input_user, product.price, product.brand);
        println!("Products relates");
        for i in product.recommendation.iter() {
            print!("{i} ")
        }
        true
    } else {
        println!("{} product not found", input_user);
        false
    }    
}    

pub fn search_product_by_info(input_user: String) {

    // This function...
    println!("Products related to your search {}.", input_user);
    let product_list = read_json();
    let mut products_found = vec![];
    for p in product_list.keys() {
        let product = product_list.get(p);
        if input_user == product.unwrap().brand || input_user == product.unwrap().category { 
            products_found.push(p);
        }
    }
    if products_found.len() == 0 {
        println!("Product {} not found", input_user)
    } else {
        for p in products_found {
            println!("{} ", p)
        }
    }
    
}

pub fn search_module(input_user: String) {

    let product_list = read_json();
    let test = input_user;
    if product_list.contains_key(&test) {
        search_product_by_name(test);
    } else {
        search_product_by_info(test);
    }
}
/*
product_list.get_mut(&item).unwrap().recommendation.insert("Mouse".to_string());
*/