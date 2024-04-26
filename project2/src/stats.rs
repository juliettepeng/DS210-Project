use std::collections::VecDeque;

pub fn bfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<Option<usize>> {
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

pub fn mean_distance(distances: &[Vec<Option<usize>>]) -> f64 {
    let mut total_distance = 0;
    let mut total_nodes = 0;

    for distance in distances {
        for d in distance {
            if let Some(dist) = d {
                total_distance += dist;
                total_nodes += 1;
            }
        }
    }

    if total_nodes > 0 {
        total_distance as f64 / total_nodes as f64
    } else {
        0.0
    }
}
