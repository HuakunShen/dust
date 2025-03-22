use du_dust::{build_directory_tree, get_largest_nodes};

fn main() {
    let tree = build_directory_tree(vec!["/Users/hk/Dev/kunkun".to_string()], false);
    // println!("{:#?}", tree);
    // let file = std::fs::File::create("tree.json").unwrap();
    // serde_json::to_writer(file, &tree).unwrap();
    let largest = get_largest_nodes(tree, 10);
    println!("{:#?}", largest);
}
