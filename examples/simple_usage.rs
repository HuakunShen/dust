// This example assumes this crate is compiled as a workspace member
// or you're running it directly from the repository

use std::env;
use std::process;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use du_dust::dir_walker;
use du_dust::display_node;
use du_dust::filter;
use du_dust::progress;
use du_dust::utils;
use du_dust::node::Node;

// For a real application, this would be:
// use dust::{node, dir_walker, filter, progress, utils, display_node};

fn main() {
    // Start the overall timer
    let start_time = Instant::now();
    
    // Get directory to analyze from command line or use current directory
    let args: Vec<String> = env::args().collect();
    let dir = if args.len() > 1 {
        args[1].clone()
    } else {
        ".".to_string()
    };
    
    // Parse additional args if provided
    let max_depth = if args.len() > 2 {
        match args[2].parse::<usize>() {
            Ok(depth) => Some(depth),
            Err(_) => None,
        }
    } else {
        None
    };
    
    let use_apparent_size = args.len() > 3 && args[3] == "--apparent-size";
    
    // Sample usage of the core functionality directly
    // First create the walk data
    let errors = progress::RuntimeErrors::default();
    let errors_for_rayon = Arc::new(Mutex::new(errors));
    let indicator = progress::PIndicator::build_me();
    
    // Set up directories to scan
    let paths = vec![dir];
    let simplified_dirs = utils::simplify_dir_names(&paths);
    
    println!("Starting scan...");
    let scan_start_time = Instant::now();
    
    // Create walk data
    let walk_data = dir_walker::WalkData {
        ignore_directories: HashSet::new(),
        filter_regex: &[],
        invert_filter_regex: &[],
        allowed_filesystems: HashSet::new(),
        filter_modified_time: None,
        filter_accessed_time: None,
        filter_changed_time: None,
        use_apparent_size, // Allow toggling between apparent size and disk usage
        by_filecount: false,
        by_filetime: &None,
        ignore_hidden: false, // Include hidden files to match CLI default
        follow_links: false,
        progress_data: indicator.data.clone(),
        errors: errors_for_rayon,
    };
    
    // Walk directories to build the tree
    let tree = dir_walker::walk_it(simplified_dirs, &walk_data);
    let scan_duration = scan_start_time.elapsed();
    println!("Scan completed in: {:.2?}", scan_duration);
    
    if tree.is_empty() {
        eprintln!("No files found");
        process::exit(1);
    }
    
    // Create the filter data to get the largest files
    let processing_start_time = Instant::now();
    let agg_data = filter::AggregateData {
        min_size: None,
        only_dir: false,
        only_file: false,
        number_of_lines: 10,
        depth: max_depth.unwrap_or(usize::MAX), // Use max_depth if provided
        using_a_filter: false,
        short_paths: false,
    };
    
    // Get the largest items
    let largest = filter::get_biggest(tree.clone(), agg_data, &None, HashSet::new());
    let processing_duration = processing_start_time.elapsed();
    
    // Calculate total size for comparison with CLI
    let total_size = if use_apparent_size {
        tree.iter().map(|node| node.size).sum::<u64>()
    } else {
        // In block mode, total is already calculated correctly in each node
        tree[0].size
    };
    
    println!("Total size: {}", format_size(total_size));
    println!("Processing completed in: {:.2?}", processing_duration);
    
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
    
    // Total time
    let total_duration = start_time.elapsed();
    println!("\nSummary:");
    println!("  Scanning time: {:.2?}", scan_duration);
    println!("  Processing time: {:.2?}", processing_duration);
    println!("  Total time: {:.2?}", total_duration);
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