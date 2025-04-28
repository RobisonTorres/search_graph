use search_graph::*;

#[test]
fn test_search_product_by_name_found() {

     // This test simulates a user searching for a product by name and retrieving associated
     // recommendations from the graph.
    let data = read_json();
    let input = "sMarfone".to_string();
    let input_user = suggest_correction(&data, input);
    let product_found = search_product_by_name(&data, input_user.as_str());
    let recs_found = bfs_recommendations(input_user.as_str(), &data.recommendation);

    assert_eq!(input_user, "Smartphone", "Input should be corrected to 'Smartphone'");
    assert!(product_found, "Product should be found by name");
    assert!(recs_found.contains("Mouse"));
    assert_eq!(recs_found.len(), 6);
}