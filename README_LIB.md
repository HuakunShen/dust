# Dust Library

This is the library version of the [Dust](https://github.com/bootandy/dust) disk usage tool. It provides the core functionality for building a tree of directory nodes with their sizes, without the CLI visualization.

## Core Data Structures

- `Node` - Represents a file or directory with size and children
- `WalkData` - Configuration for walking a directory tree
- `Operator` - Operators for time-based filtering
- `FileTime` - File time attributes (modified, accessed, changed)
- `DisplayNode` - Node representation for display purposes

## Example Usage

```rust
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

// Import the dust library
use dust::{
    dir_walker::{walk_it, WalkData},
    filter::{AggregateData, get_biggest},
    node::FileTime,
    progress::{PIndicator, RuntimeErrors},
    utils::simplify_dir_names,
};

fn main() {
    // Set up directories to scan
    let paths = vec![".".to_string()]; // Scan current directory
    let simplified_dirs = simplify_dir_names(&paths);
    
    // Create progress tracking
    let errors = RuntimeErrors::default();
    let errors_for_rayon = Arc::new(Mutex::new(errors));
    let indicator = PIndicator::build_me();
    
    // Create walk data configuration
    let walk_data = WalkData {
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
    
    // Walk the directory tree
    let tree = walk_it(simplified_dirs, &walk_data);
    
    // Set up filtering for largest items
    let agg_data = AggregateData {
        min_size: None,
        only_dir: false,
        only_file: false,
        number_of_lines: 10, // Get top 10 items
        depth: usize::MAX,
        using_a_filter: false,
        short_paths: false,
    };
    
    // Get the largest items
    let largest = get_biggest(tree, agg_data, &None, HashSet::new());
    
    // Stop the progress indicator
    indicator.stop();
    
    // Process the results
    if let Some(root) = largest {
        println!("Largest items:");
        print_results(&root);
    }
}

// Helper function to print results
fn print_results(node: &dust::display_node::DisplayNode) {
    println!("{}: {} bytes", node.name.display(), node.size);
    
    for child in &node.children {
        println!("  {}: {} bytes", child.name.display(), child.size);
    }
}
```

## Features

- Build a tree of files/directories with their sizes
- Filter by file patterns, sizes, or timestamps
- Identify largest files in a directory
- Deduplicate files with same inodes
- Flexible configuration options

## Integration

This library can be integrated into any Rust application that needs disk usage analysis functionality. 