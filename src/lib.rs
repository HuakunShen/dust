pub mod node;
pub mod dir_walker;
pub mod filter;
pub mod utils;
pub mod platform;
pub mod progress;
pub mod display_node;
pub mod config;
pub mod display;

// Re-export core types for direct usage
pub use node::{Node, FileTime};
pub use dir_walker::{WalkData, Operator, walk_it};
pub use filter::{AggregateData, get_biggest};

// A simplified API for users who just want a tree of nodes with sizes
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use regex::Regex;

/// A simplified API for building a tree of directory nodes
pub fn build_directory_tree(
    directories: Vec<String>,
    max_depth: Option<usize>,
    ignore_hidden: bool,
) -> Vec<Node> {
    let simplified_dirs = utils::simplify_dir_names(&directories);
    
    // Create minimal walk data
    let errors = progress::RuntimeErrors::default();
    let errors_for_rayon = Arc::new(Mutex::new(errors));
    let indicator = progress::PIndicator::build_me();
    
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
        ignore_hidden,
        follow_links: false,
        progress_data: indicator.data.clone(),
        errors: errors_for_rayon,
    };
    
    // Walk the directories
    dir_walker::walk_it(simplified_dirs, &walk_data)
}

/// Get largest nodes from a tree with a specified limit
pub fn get_largest_nodes(
    nodes: Vec<Node>,
    number_of_nodes: usize, 
) -> Option<display_node::DisplayNode> {
    if nodes.is_empty() {
        return None;
    }
    
    // Create minimal aggregate data
    let agg_data = filter::AggregateData {
        min_size: None,
        only_dir: false,
        only_file: false,
        number_of_lines: number_of_nodes,
        depth: usize::MAX,
        using_a_filter: false,
        short_paths: false,
    };
    
    filter::get_biggest(nodes, agg_data, &None, HashSet::new())
} 