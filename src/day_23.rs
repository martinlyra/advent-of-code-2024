use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    sync::Arc,
};

use itertools::Itertools;
use regex::Regex;

struct Graph<T> {
    nodes: HashSet<Arc<T>>,
    edges: HashMap<Arc<T>, HashSet<Arc<T>>>,
}

impl<T: std::cmp::Eq + std::hash::Hash + ToOwned> Graph<T> {
    fn add_edge(&mut self, node_a: T, node_b: T) {
        let a = Arc::new(node_a);
        let b = Arc::new(node_b);
        self._add_to_edges(&a, &b);
        self._add_to_edges(&b, &a);
        self.nodes.insert(a);
        self.nodes.insert(b);
    }

    fn _add_to_edges(&mut self, node_a: &Arc<T>, node_b: &Arc<T>) {
        self.edges
            .entry(node_a.clone())
            .or_insert(HashSet::new())
            .insert(node_b.clone());
    }

    fn find_maximal_cliques(&self) -> Vec<HashSet<Arc<T>>> {
        self._bron_kerbosch_1(HashSet::new(), self.nodes.clone(), HashSet::new())
    }

    fn _bron_kerbosch_1(
        &self,
        r: HashSet<Arc<T>>,
        mut p: HashSet<Arc<T>>,
        mut x: HashSet<Arc<T>>,
    ) -> Vec<HashSet<Arc<T>>> {
        if p.is_empty() && x.is_empty() {
            return vec![r];
        }
        let mut cliques = Vec::new();
        for v in p.clone() {
            let neighbours = &self.edges[&v];
            cliques.extend(self._bron_kerbosch_1(
                r.union(&HashSet::from([v.clone()])).cloned().collect(),
                p.clone().intersection(neighbours).cloned().collect(),
                x.intersection(neighbours).cloned().collect(),
            ));
            p.remove(&v);
            x.insert(v.clone());
        }
        cliques
    }
}

fn main() {
    let test = read_input("./input/day_23.test.txt");
    let input = read_input("./input/day_23.txt");

    println!("First part test answer: {} == 7", part_1(&test));
    println!("First part answer: {}", part_1(&input));

    println!("Second part test answer: {}", part_2(&test));
    println!("Second part answer: {}", part_2(&input));
}

fn part_1(graph: &Graph<String>) -> usize {
    graph
        .nodes
        .iter()
        .filter(|node| node.starts_with('t'))
        .map(|node| {
            let mut triangles: Vec<Vec<&String>> = Vec::new();
            let connections = Vec::from_iter(graph.edges[node].iter());
            let n = connections.len();
            for i in 0..n {
                for j in i..n {
                    let b = connections[i];
                    let c = connections[j];
                    if graph.edges[b].contains(c) {
                        triangles.push(vec![node, b, c]);
                    }
                }
            }
            triangles
        })
        .flatten()
        .map(|mut v| {
            v.sort();
            v
        })
        .unique()
        .count()
}

fn part_2(graph: &Graph<String>) -> String {
    graph
        .find_maximal_cliques()
        .iter()
        .sorted_by(|a, b| Ord::cmp(&a.len(), &b.len()))
        .rev()
        .next()
        .unwrap()
        .iter()
        .sorted()
        .join(",")
}

fn read_input(file_path: &str) -> Graph<String> {
    let edge_pattern = Regex::new(r"(\w+)-(\w+)").unwrap();
    read_to_string(file_path)
        .unwrap()
        .trim()
        .split("\n")
        .filter_map(|line| edge_pattern.captures(line))
        .map(|captured| {
            let (_, [node_a, node_b]) = captured.extract();
            (node_a, node_b)
        })
        .fold(
            Graph {
                nodes: HashSet::new(),
                edges: HashMap::new(),
            },
            |mut graph, edge| {
                graph.add_edge(edge.0.to_owned(), edge.1.to_owned());
                graph
            },
        )
}
