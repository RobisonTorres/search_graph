use std::io;
use ucfirst::ucfirst;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use strsim::levenshtein;

#[derive(Debug, Deserialize)]
pub struct Product {
    pub price: f64,
    pub brand: String,
    pub category: String,
}

#[derive(Debug, Deserialize)]
pub struct TopLevel {

    #[serde(rename = "Product")]
    pub product: HashMap<String, Product>,

    #[serde(rename = "Recommendation")]
    pub recommendation: HashMap<String, HashSet<String>>,
}

pub fn take_input_user() -> String {

    // This function takes user's input.
    println!("Search for product...");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input) 
        .expect("Unable to read Stdin");
    let input = ucfirst(input.trim());
    input.to_string()
}

pub fn read_json_products() -> HashMap<String, Product> {

    // This function...
    let file = File::open("products.json").expect("Failed to open file");
    let reader = BufReader::new(file);
    let data: TopLevel = serde_json::from_reader(reader).expect("Failed to parse JSON");
    data.product
}

pub fn read_json_recommendations() -> HashMap<String, HashSet<String>> {

    // This function...
    let file = File::open("products.json").expect("Failed to open file");
    let reader = BufReader::new(file);
    let data: TopLevel = serde_json::from_reader(reader).expect("Failed to parse JSON");
    data.recommendation
}

fn search_product_by_name(input_user: String) -> bool {

    // This function try to find a product by its exact name and prints its details if found.
    let product_list: HashMap<String, Product> = read_json_products();
    let recommendation_list: HashMap<String, HashSet<String>> = read_json_recommendations();
    if let Some(product) = product_list.get(&input_user) {
        println!("-----------------------------------");
        println!("Product Found: \n");
        println!("Name: {}", input_user);
        println!("Price: ${:.2}", product.price);
        println!("Brand: {}", product.brand);
        print!("\nSee also: ");
        if let Some(recommendations) = recommendation_list.get(&input_user) {
            for r in recommendations {
                print!("{} ", r);
            }
        } else {
            println!("No recommendations found for {}", input_user);
        }      
        println!("\n-----------------------------------");
        true
    } else { 
        println!("\nRelated products for '{}'", input_user);
        println!("-----------------------------------");
        false 
    }    
}    

fn search_product_by_info(input_user: String) {

    // This function searches for products based on brand or category and prints matching product names.
    let product_list = read_json_products();
    let mut found = false;
    for product_find in product_list.keys() {
        let product = product_list.get(product_find);
        if input_user == product.unwrap().brand || input_user == product.unwrap().category { 
            println!("{} ", product_find);
            found = true;
        }
    }   
    if !found {
        let spell_check = suggest_correction(input_user);
        println!("Did you mean {}?", spell_check);
        search_product_by_name(spell_check);
    }   
    println!("-----------------------------------");
}

pub fn search_module(input_user: String) {

    // This function attempts to find a product by its name. If not found, searches by brand or category.
    if !search_product_by_name(input_user.clone()) {
        search_product_by_info(input_user);
    }
}

fn suggest_correction(misspelled: String) -> String {
    
    // This function finds the correct word based on the minimum Levenshtein distance.
    let product_list: HashMap<String, Product> = read_json_products();
    let dictionary = product_list.keys();
    dictionary
        .min_by_key(|word| levenshtein(&misspelled, word))
        .unwrap_or(&misspelled).to_string()
}