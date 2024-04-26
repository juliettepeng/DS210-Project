use std::fs::File;
use std::io::{self, BufRead};
//use rand::Rng;
use rand::prelude::SliceRandom;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;


fn read_file(path: &str)-> Vec<(usize, usize)>{
    let points = File::open(path).expect("Could not open file");
    let lines = io::BufReader::new(points).lines();
    let mut graph: Vec<(usize, usize)> = Vec::new();

    for (idx, line) in lines.enumerate(){
        let string = line.expect("Error");
        let parts: Vec<&str> = string.trim().split('\t').collect();
        let x = parts[0].parse::<usize>().unwrap();
        let y = parts[1].parse::<usize>().unwrap();
        graph.push((x, y));
    }
    return graph;
}

fn take_random_n<T>(data: &Vec<T>, n: usize) -> Vec<T>
where
    T: Clone,
{
    let mut rng = rand::thread_rng();
    let mut shuffled_data = data.clone();
    shuffled_data.shuffle(&mut rng);
    shuffled_data.into_iter().take(n).collect()
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

fn bfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<Option<usize>> {
    let mut distance: Vec<Option<usize>> = vec![None; graph.len()];
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(start);
    distance[start] = Some(0);

    while let Some(node) = queue.pop_front() {
        for &neighbor in &graph[node] {
            if distance[neighbor].is_none() {
                distance[neighbor] = Some(distance[node].unwrap() + 1);
                queue.push_back(neighbor);
            }
        }
    }
    distance
}


fn main() {
    let graph = read_file("roadNet-PA.txt");
    let n = 10_000;
    let random_graph = take_random_n(&graph, n);
    let unique_graph = unique_nodes(&random_graph);
    let adj = adjacency_list(&random_graph, unique_graph);

    //println!("{:?}", random_graph);
    println!("Length of graph: {}", random_graph.len());
    //println!("{:?}", adj);

    for i in 0..adj.len() {
        println!("Distances from node {}", i);
        let distances = bfs(&adj, i);
        for (node, dist) in distances.iter().enumerate() {
            if let Some(d) = dist {
                println!("To {}: {}", node, d);
            }
        }
    }

}
