use super::*;

#[test]
fn test_search_product_by_name_found() {

    // Tests if search_product_by_name correctly finds an existing product by its name ("Blender").
    let data = read_json();
    let found = search_product_by_name(&data, "Blender");
    assert!(found, "Product should be found by name");
}

#[test]
fn test_search_product_by_name_not_found() {

    // Tests if search_product_by_name correctly returns false when the product name ("Watch") does not exist.
    let data = read_json();
    let found = search_product_by_name(&data, "Watch");
    assert!(!found, "Product should not be found");
}

#[test]
fn test_search_product_by_info_brand_found() {

    // Tests if search_product_by_info finds a product by matching the brand ("Nike").
    let data = read_json();
    let found = search_product_by_info(&data, "Nike");
    assert!(found, "Brand should be found");
}

#[test]
fn test_search_product_by_info_brand_not_found() {

    // Tests if search_product_by_info returns false when the brand ("Jeep") does not exist in the data.
    let data = read_json();
    let found = search_product_by_info(&data, "Jeep");
    assert!(!found, "Brand should not be found");
}

#[test]
fn test_search_product_by_info_category_found() {

    // Tests if search_product_by_info finds a product by matching the category ("Home Appliances").
    let data = read_json();
    let found = search_product_by_info(&data, "Home Appliances");
    assert!(found, "Category should be found");
}

#[test]
fn test_search_product_by_info_category_not_found() {

    // Tests if search_product_by_info returns false when the category ("Dairy") does not exist in the data.
    let data = read_json();
    let found = search_product_by_info(&data, "Dairy");
    assert!(!found, "Category should not be found");
}

#[test]
fn test_bfs_recommendations() {

    // Tests if bfs_recommendations returns the expected products related to "Smartphone".
    // It checks that "Mouse" is among the recommendations and that there are exactly 6 recommended items.
    let data = read_json();
    let recs = bfs_recommendations("Smartphone", &data.recommendation);
    assert!(recs.contains("Mouse"));
    assert_eq!(recs.len(), 6);
}

#[test]
fn test_suggest_correction_typo() {

    // Tests if suggest_correction can correctly suggest the right word ("Laptop") for a typo ("Leptóp").
    let data = read_json();
    let correction = suggest_correction(&data, "Leptóp".to_string());
    assert_eq!(correction, "Laptop");
}