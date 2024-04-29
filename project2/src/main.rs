use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::collections::HashMap;
mod stats;
use stats::{mean_distance, bfs, std_dev}; 

fn read_file(path: &str) -> Vec<(usize, usize)> {
    let points = File::open(path).expect("Could not open file");
    let lines = io::BufReader::new(points).lines();
    let mut graph: Vec<(usize, usize)> = Vec::new();

    for line in lines {
        let string = line.expect("Error reading line");
        let parts: Vec<&str> = string.trim().split(',').collect(); // Split by comma
        if parts.len() == 2 {
            if let Ok(x) = parts[0].parse::<usize>() {
                if let Ok(y) = parts[1].parse::<usize>() {
                    graph.push((x, y));
                }
            }
        }
    }
    return graph;
}

fn unique_nodes(graph: &Vec<(usize, usize)>) -> HashSet<usize> {
    let mut hash_set: HashSet<usize> = HashSet::new();
    for (i, j) in graph.iter() {
        hash_set.insert(*i);
        hash_set.insert(*j);
    }
    return hash_set;
}

fn adjacency_list(graph: &Vec<(usize,usize)>, hash_set: HashSet<usize>) -> Vec<Vec<usize>> {
    let new_num = hash_set.len();
    let new_vec: Vec<&usize> = hash_set.iter().collect();
    let mut adj: Vec<Vec<usize>> = vec![vec![];new_num];
    let mut node_map: HashMap<usize, usize> = HashMap::new();
    for (i, j) in new_vec.iter().enumerate(){
        node_map.insert(**j,i);
    }
    for (source, destination) in graph {
        adj[node_map[&source]].push(node_map[&destination]);
    }
    for (v, n) in adj.iter().enumerate(){
        println!("Vertex {}: {:?}", v, n);
    }
    return adj
}

fn main() {
    let graph = read_file("euroroad.csv");
    let unique_graph = unique_nodes(&graph);
    let adj = adjacency_list(&graph, unique_graph);
    println!("Length of graph: {}", graph.len());
    
    for i in 0..adj.len() {
        println!("Distance from node {}", i);
        //let distances = bfs(&adj, i);
        let distances = bfs(&adj, i);
        for (node, dist) in distances.iter().enumerate() {
            if let Some(d) = dist {
                println!("To {}: {}", node, d);
            }
        }
    }
   
    let mut distances: Vec<Vec<Option<usize>>> = Vec::new();
    for i in 0..adj.len() {
    println!("Distance from node {}", i);
    let distance = bfs(&adj, i);
    distances.push(distance);
    for (node, dist) in distances.last().unwrap().iter().enumerate() {
        if let Some(d) = dist {
            println!("To {}: {}", node, d);
        }
    }
}

let mean_dist = stats::mean_distance(&distances);
println!("Mean Distance: {}", mean_dist);

let sd = stats::std_dev(&distances, mean_dist);
println!("Standard Deviation: {}", sd);

test_bfs_zeros();
    
}

fn test_adjacency_list(graph: &Vec<(usize,usize)>, unique_nodes: HashSet<usize>) -> Vec<Vec<usize>> {
    let new_num = unique_nodes.len();
    let new_vec: Vec<&usize> = unique_nodes.iter().collect();
    let mut adj: Vec<Vec<usize>> = vec![vec![];new_num];
    let mut node_map: HashMap<usize, usize> = HashMap::new();
    for (i, j) in new_vec.iter().enumerate(){
        node_map.insert(**j,i);
    }
    for (source, destination) in graph {
        adj[node_map[&source]].push(node_map[&destination]);
    }
    adj
}

fn test_bfs_zeros() -> i32{
    let file = "test.txt";
    let graph = read_file(file);
    let unique_nodes = unique_nodes(&graph);
    let test_adj = test_adjacency_list(&graph, unique_nodes.clone());

    let mut passed = true;

    for (i,node) in unique_nodes.iter().enumerate(){
        let distances = bfs(&test_adj, i);
        if distances[i] != Some(0) {
            println!("Node {}: Expected 0, Got {:?}", node, distances[i]);
            passed = false;
        } else {
            println!("Distance from node {}", i);
            println!("To {}: {}", i, 0);
        }
     }
     if passed {
        println!("All nodes passed!");
        0
     }  else {
        println!("Some nodes failed.");
        1
     }
}