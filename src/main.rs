use search_graph::*;

fn main() {
    let data: TopLevel = read_json();
    search_module(&data, &take_input_user());
}