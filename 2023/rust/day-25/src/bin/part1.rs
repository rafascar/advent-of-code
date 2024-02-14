use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graph::{NodeIndex, UnGraph};
use rustworkx_core::Result;

use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    let answer = process(input);
    dbg!(answer);
}

fn process(input: &str) -> String {
    let graph = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(':').expect("should be a: b");
            (
                a.into(),
                b.split_whitespace()
                    .map(|s| s.trim().into())
                    .collect::<HashSet<String>>(),
            )
        })
        .collect::<HashMap<String, HashSet<String>>>();

    let mut ungraph: UnGraph<(), ()> = UnGraph::new_undirected();

    let mut nodes: HashMap<String, NodeIndex> = HashMap::new();

    let mut node_names = HashSet::new();
    for (a, b) in graph.iter() {
        node_names.insert(a);
        for c in b.iter() {
            node_names.insert(c);
        }
    }

    for n in node_names.iter() {
        nodes.insert(n.to_string(), ungraph.add_node(()));
    }

    let mut edges = vec![];
    for (a, b) in graph.iter() {
        for c in b.iter() {
            edges.push((nodes[a], nodes[c]));
        }
    }

    ungraph.extend_with_edges(edges);
    let min_cut_res: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&ungraph, |_| Ok(1));

    let s = min_cut_res.unwrap().unwrap().1.len();
    let t = node_names.len() - s;

    (s * t).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

        let result = process(input);
        assert_eq!(result, "54".to_string());
    }
}
