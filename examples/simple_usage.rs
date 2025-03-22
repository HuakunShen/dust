// This example assumes this crate is compiled as a workspace member
// or you're running it directly from the repository

use std::env;
use std::process;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use du_dust::dir_walker;
use du_dust::display_node;
use du_dust::filter;
use du_dust::progress;
use du_dust::utils;

// For a real application, this would be:
// use dust::{node, dir_walker, filter, progress, utils, display_node};

fn main() {
    // Get directory to analyze from command line or use current directory
    let args: Vec<String> = env::args().collect();
    let dir = if args.len() > 1 {
        args[1].clone()
    } else {
        ".".to_string()
    };
    
    // Sample usage of the core functionality directly
    // First create the walk data
    let errors = progress::RuntimeErrors::default();
    let errors_for_rayon = Arc::new(Mutex::new(errors));
    let indicator = progress::PIndicator::build_me();
    
    // Set up directories to scan
    let paths = vec![dir];
    let simplified_dirs = utils::simplify_dir_names(&paths);
    
    // Create walk data
    let walk_data = dir_walker::WalkData {
        ignore_directories: HashSet::new(),
        filter_regex: &[],
        invert_filter_regex: &[],
        allowed_filesystems: HashSet::new(),
        filter_modified_time: None,
        filter_accessed_time: None,
        filter_changed_time: None,
        use_apparent_size: false,
        by_filecount: false,
        by_filetime: &None,
        ignore_hidden: true,
        follow_links: false,
        progress_data: indicator.data.clone(),
        errors: errors_for_rayon,
    };
    
    // Walk directories to build the tree
    let tree = dir_walker::walk_it(simplified_dirs, &walk_data);
    
    if tree.is_empty() {
        eprintln!("No files found");
        process::exit(1);
    }
    
    // Create the filter data to get the largest files
    let agg_data = filter::AggregateData {
        min_size: None,
        only_dir: false,
        only_file: false,
        number_of_lines: 10,
        depth: usize::MAX,
        using_a_filter: false,
        short_paths: false,
    };
    
    // Get the largest items
    let largest = filter::get_biggest(tree, agg_data, &None, HashSet::new());
    
    match largest {
        Some(root) => {
            println!("Largest items in directory:");
            print_node(&root, 0);
        }
        None => {
            println!("No files found");
        }
    }
    
    // Stop the progress indicator
    indicator.stop();
}

fn print_node(node: &display_node::DisplayNode, indent: usize) {
    let indent_str = " ".repeat(indent * 2);
    println!("{}{}: {}", indent_str, node.name.display(), format_size(node.size));
    
    // Print children
    for child in &node.children {
        print_node(child, indent + 1);
    }
}

/// Format bytes into a human-readable string with appropriate units
fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;
    
    if size < KB {
        format!("{} B", size)
    } else if size < MB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else if size < GB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size < TB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else {
        format!("{:.2} TB", size as f64 / TB as f64)
    }
} 