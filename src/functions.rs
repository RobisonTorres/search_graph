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

fn read_json() -> HashMap<String, Product> {

    // This function reads a JSON file and returns a HashMap of product names to Product structs.
    let file = File::open("products.json").expect("Failed to open file");
    let reader = BufReader::new(file);  
    let product_list: HashMap<String, Product> = serde_json::from_reader(reader).expect("REASON"); 
    product_list
}

fn search_product_by_name(input_user: String) -> bool {

    // This function try to find a product by its exact name and prints its details if found.
    let product_list = read_json();
    if let Some(product) = product_list.get(&input_user) {
        println!("{} - ${:.2} - {}.", input_user, product.price, product.brand);
        println!("Products related");
        for i in product.recommendation.iter() {
            print!("{i} ")
        }
        true
    } else {
        println!("Products related to your search {}.", input_user);
        false
    }    
}    

/*fn search_product_by_info(input_user: String) {

    // This function searches for products based on brand or category and prints matching product names.
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
    
}*/

fn search_product_by_info(input_user: String) {

    // This function searches for products based on brand or category and prints matching product names.
    let product_list = read_json();
    for p in product_list.keys() {
        let product = product_list.get(p);
        if input_user == product.unwrap().brand || input_user == product.unwrap().category { 
            println!("{} ", p);
        }
    }   
}

pub fn search_module(input_user: String) {

    // This function attempts to find a product by its name. If not found, searches by brand or category.
    if !search_product_by_name(input_user.clone()) {
        search_product_by_info(input_user);
    }
}
/*
product_list.get_mut(&item).unwrap().recommendation.insert("Mouse".to_string());
*/