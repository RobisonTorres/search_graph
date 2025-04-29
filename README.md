# Optimized Search System for Product Catalog - MegaStore

MegaStore, a retail giant with an expansive e-commerce operation, offers millions of products across categories like electronics, fashion, home decor, and food. With such a large catalog, the company faces a major challenge: helping customers find relevant products quickly and effectively.

This project implements an optimized search and recommendation system in Rust that leverages **graph-based algorithms** to improve product discovery. It enhances search precision, handles user typos via input correction, and provides related product recommendations through breadth-first traversal (BFS) of a graph.

## Modules & Components

- **main.rs**: CLI interaction and runtime execution
- **functions.rs**: Core algorithms including graph search and fuzzy matching
- **lib.rs**: Exports functionality for modular use and testing
- **products.json**: Product catalog and similarity edges in JSON format

## Algorithms and Data Structures

- **Graph Structure**: Products are nodes; edges represent related items (e.g., similar brand or category)
- **Breadth-First Search (BFS)**: Used for traversing the product graph to find and recommend related products
- **Hash Maps (via `HashMap`)**: Provide fast access to product data for search and recommendation
- **Levenshtein Distance**: Used to suggest corrections for misspelled inputs

## Performance and Scalability Considerations

- **Efficiency**: The BFS algorithm ensures that recommendations are generated in linear time relative to the number of products visited.
- **Scalability**: Use of hash maps enables constant-time lookups, suitable for scaling with millions of products.
- **Fault-Tolerance**: Input correction via fuzzy matching improves user experience and mitigates data entry errors.
- **Test Coverage**: Unit and integration tests ensure consistent performance under edge cases and larger datasets.

## Project Structure

├── Cargo.toml # Project dependencies and metadata  
├── products.json # JSON file containing product and recommendation data  
├── src/  
│   ├── functions.rs # Core functions for search and recommendations  
│   ├── lib.rs # Library module  
│   ├── main.rs # Entry point of the application  
│   ├── tests.rs # Unit tests for core functions  
├── tests/  
│   ├── integration_test.rs # Integration tests for the application  
├── target/ # Build artifacts (ignored by Git)  
└── README.md # Project documentation  

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (version 1.70 or later)

### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/RobisonTorres/search_graph.git
   cd search_graph
   ```

2. Build the project:
   ```sh
   cargo build
   ```

If the build fails, temporarily disable your antivirus software and try again, as it may interfere with the build process.

3. Run the tests:
   ```sh
   cargo test
   ``` 
    Or
   ```sh 
   cargo test --test integration_test
   ```

## Usage

Run the application:
```sh
cargo run
```

Enter a product name, brand, or category when prompted. The application will display the product details and recommendations.

### Example

Input:
```
Smartphone
```
Output:
```
-----------------------------------
Name: Smartphone
Price: $699.99
Brand: Samsung

See also: 
- Mouse
- Headphones
- Laptop
- Monitor
- Bluetooth Speaker
- Smartwatch
```

## Dependencies

- `serde`: For JSON serialization and deserialization.
- `serde_json`: For working with JSON data.
- `strsim`: For calculating Levenshtein distance.
- `Inflector`: For string case conversions.

## License

This project is licensed under the MIT License. See the LICENSE file for details.