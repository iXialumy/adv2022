use std::collections::HashMap;

use petgraph::algo::dijkstra;
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::prelude::Graph;
use petgraph::visit::NodeRef;

pub fn day12() {
    let input = include_str!("../../input/12.txt");
    let graph = create_graph(input);
    day12a(&graph);
    day12b(&graph);
}

fn day12a(graph: &Graph<Weight, usize>) {
    let goal = find_goal(graph);
    let start = find_start(graph);
    let dij = dijkstra(&graph, start, None, |_| 1);

    let dist = dij.get(&goal.unwrap()).unwrap();
    println!("Day12a: {dist:?}");
}

fn day12b(graph: &Graph<Weight, usize>) {
    let goal = find_goal(graph);
    let starts = find_starts(graph);

    let min = starts
        .iter()
        .filter_map(|start| {
            let dij = dijkstra(&graph, *start, None, |_| 1);
            let dist = dij.get(&goal.unwrap())?;
            Some(*dist)
        })
        .min()
        .unwrap();

    println!("Day12b: {min}");
}

fn find_start(graph: &Graph<(char, (usize, usize)), usize>) -> NodeIndex {
    graph
        .node_indices()
        .find(|index| graph.node_weight(*index).unwrap().0 == 'S')
        .unwrap()
}

fn find_starts(graph: &Graph<(char, (usize, usize)), usize>) -> Vec<NodeIndex> {
    let starts: Vec<NodeIndex> = graph
        .node_indices()
        .filter(|index| graph.node_weight(*index).unwrap().0 == 'a')
        .collect();
    starts
}

fn find_goal(graph: &Graph<(char, (usize, usize)), usize>) -> Option<NodeIndex> {
    graph
        .node_indices()
        .find(|index| graph.node_weight(*index).unwrap().0 == 'E')
}

fn create_graph(input: &str) -> Graph<Weight, usize> {
    build_graph(&create_matrix(input))
}

fn create_matrix(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

type Weight = (char, (usize, usize));

fn build_graph(matrix: &Vec<Vec<char>>) -> Graph<Weight, usize> {
    let mut g = Graph::<Weight, usize>::new();

    let mut nodemap = HashMap::new();

    (0..matrix.len()).for_each(|i| {
        for j in 0..matrix[i].len() {
            let coords = (i, j);
            let node = g.add_node((matrix[i][j], coords));
            nodemap.insert(coords, node);
        }
    });

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let coords = (i, j);
            let height = matrix[i][j];
            let node = nodemap.get(&coords).unwrap().to_owned();
            if i < matrix.len() - 1 {
                let coords2 = (i + 1, j);
                let height2 = matrix[i + 1][j];
                let node2 = nodemap.get(&coords2).unwrap().to_owned();
                if connected(height, height2) {
                    g.add_edge(node, node2, 1);
                    g.add_edge(node2, node, 1);
                } else {
                    if height > height2 {
                        g.add_edge(node, node2, 1);
                    }
                    if height2 > height {
                        g.add_edge(node2, node, 1);
                    }
                }
            }
            if j < matrix[i].len() - 1 {
                let coords2 = (i, j + 1);
                let height2 = matrix[i][j + 1];
                let node2 = nodemap.get(&coords2).unwrap().to_owned();
                if connected(height, height2) {
                    g.add_edge(node, node2, 1);
                    g.add_edge(node2, node, 1);
                } else {
                    if height > height2 {
                        g.add_edge(node, node2, 1);
                    }
                    if height2 > height {
                        g.add_edge(node2, node, 1);
                    }
                }
            }
        }
    }

    g
}

fn connected(a: char, b: char) -> bool {
    if a == 'S' || b == 'S' {
        true
    } else if a == 'E' || b == 'E' {
        a == 'z' || b == 'z'
    } else {
        (a as u8).abs_diff(b as u8) <= 1
    }
}
