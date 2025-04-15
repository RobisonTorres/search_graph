use std::collections::HashMap;
// Option
/*
struct Product {
    price: f64,
    brand: String,
}
*/
fn main() {

    //let mut product_list: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut product_list: HashMap<String, [String; 2]> = HashMap::new();

    product_list.insert(String::from("Mouse"), 
    ["Price: 19.99".to_string(), "Brand: Toyota".to_string()]);

    let item = String::from("Mouse");

    if let Some(info) = product_list.get(&item) {
        println!("{item} - {} - {}", info[0], info[1]);
    } else {
            println!("Item '{}' not found.", item);
    }
}

/*
// Representação do grafo usando HashMap
type Graph = HashMap<i32, Vec<i32>>;

fn main() {
    let mut grafo: Graph = HashMap::new();

    // Adicionar nós (chaves)
    grafo.insert(1, vec![]);
    grafo.insert(2, vec![]);
    grafo.insert(3, vec![]);

    // Adicionar arestas (vizinhos)
    grafo.get_mut(&1).unwrap().push(2);
    grafo.get_mut(&2).unwrap().push(1);
    grafo.get_mut(&2).unwrap().push(3);
    grafo.get_mut(&3).unwrap().push(2);

    // Percorrer o grafo e imprimir os vizinhos
    for (no, vizinhos) in &grafo {
        println!("No {}: Vizinhos {:?}", no, vizinhos);
    }
}
*/