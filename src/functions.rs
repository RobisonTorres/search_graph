use std::{io, io::BufReader, fs::File};
use std::collections::{HashMap, HashSet, VecDeque};
use serde::{Serialize, Deserialize};
use strsim::levenshtein;
use ucfirst::ucfirst;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub price: f64,
    pub brand: String,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopLevel {
    #[serde(rename = "Product")]
    pub product: HashMap<String, Product>,
    #[serde(rename = "Recommendation")]
    pub recommendation: HashMap<String, HashSet<String>>,
}

pub fn read_json() -> TopLevel {

    // This function reads a JSON file and returns a list of products with its recommendations.
    let file = File::open("products.json").expect("Failed to open file");
    let reader = BufReader::new(file);
    let data: TopLevel = serde_json::from_reader(reader).expect("Failed to parse JSON");
    data
}

pub fn take_input_user() -> String {

    // This function takes user's input.
    println!("-----------------------------------");
    println!("--------Search for product---------");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input) 
        .expect("Unable to read Stdin");
    let input = ucfirst(input.trim());
    input.to_string()
}

pub fn search_module(data: &TopLevel, input_user: &str) {

    // This function attempts to find a product by its name. If not found, searches by brand or category.
    if !search_product_by_name(&data, input_user) {
        search_product_by_info(&data, input_user);
    }
}

fn search_product_by_name(data: &TopLevel, input_user: &str) -> bool {

    // This function tries to find a product by its exact name and prints its details if found.
    let product_list: &HashMap<String, Product> = &data.product;
    if let Some(product) = product_list.get(input_user) {
        println!("-----------------------------------");
        println!("Name: {}", input_user);
        println!("Price: ${:.2}", product.price);
        println!("Brand: {}", product.brand);
        print!("\nSee also: \n");
        let product_recommendation = bfs_recommendations(input_user, &data.recommendation);
        for r in product_recommendation {
            println!("- {}", r);
        }
        println!("\n-----------------------------------");
        true
    } else { 
        false 
    }    
}    

fn search_product_by_info(data: &TopLevel, input_user: &str) {

    // This function searches for products based on brand or category and prints matching product names.
    let mut found = false;
    println!("\nProducts related to: '{}'", input_user);
    for product_find in data.product.keys() {
        if let Some(product) = data.product.get(product_find) {
            if input_user == product.brand || input_user == product.category { 
                println!("{} ", product_find);
                found = true;
            }
        }  
    }
    if !found {
        println!("\nNo products found matching '{}'", input_user);
        let spell_check = suggest_correction(data, input_user.to_string());
        println!("Did you mean {}?", spell_check);
        search_product_by_name(data, &spell_check);
    }   
    println!("-----------------------------------");
}

pub fn bfs_recommendations(start: &str, graph: &HashMap<String, HashSet<String>>) -> HashSet<String> {

    // This function executes a BFS search on the "Recommendation" field to find related products.
    let depth = 2;
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut recommendation = HashSet::new();
    queue.push_back((start.to_string(), 0)); 
    visited.insert(start.to_string()); 
    while let Some((current, level)) = queue.pop_front() { 
        if level > 0 {
            recommendation.insert(current.clone());  
        }
        if level < depth {
            if let Some(neighbors) = graph.get(&current) { 
                for neighbor in neighbors {                         
                    if !visited.contains(neighbor) {
                        queue.push_back((neighbor.clone(), level + 1));
                        visited.insert(neighbor.clone());  
                    }
                }
            }
        }
    }
    recommendation
}

fn suggest_correction(data: &TopLevel, misspelled: String) -> String {
    
    // This function finds the correct word based on the minimum Levenshtein distance.
    let dictionary = data.product.keys();
    dictionary
        .min_by_key(|word| levenshtein(&misspelled, word))
        .unwrap_or(&misspelled).to_string()
}