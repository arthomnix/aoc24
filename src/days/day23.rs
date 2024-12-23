use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> HashMap<String, HashSet<String>> {
    let mut graph = HashMap::new();
    for line in input.lines() {
        let (l, r) = line.split_once("-").unwrap();
        let (l, r) = (l.to_string(), r.to_string());
        graph.entry(l.clone()).and_modify(|s: &mut HashSet<_>| { s.insert(r.clone()); }).or_insert(HashSet::from([r.clone()]));
        graph.entry(r).and_modify(|s| { s.insert(l.clone()); }).or_insert(HashSet::from([l]));
    }
    graph
}

fn find_triangles(mut graph: HashMap<String, HashSet<String>>) -> i32 {
    // Chiba and Nishizeki (1985)
    let mut vertices = graph.keys().cloned().collect::<Vec<_>>();
    vertices.sort_unstable_by(|a, b| graph[b.as_str()].len().cmp(&graph[a.as_str()].len()));

    let mut count = 0;

    for i in 0..vertices.len() - 2 {
        let mut skip = HashSet::new();
        for u in &graph[&vertices[i]] {
            if !skip.contains(&u) {
                for w in &graph[u] {
                    if graph[&vertices[i]].contains(w)
                        && (vertices[i].starts_with('t') || u.starts_with('t') || w.starts_with('t'))
                    {
                        count += 1;
                    }
                }
                skip.insert(u);
            }
        }

        graph.remove(&vertices[i]);
        for v in graph.values_mut() {
            v.remove(&vertices[i]);
        }
    }

    count / 2
}


pub(crate) fn part1(input: String) {
    let graph = parse(&input);
    println!("{}", find_triangles(graph));
}

fn bron_kerbosch(r: HashSet<String>, mut p: HashSet<String>, mut x: HashSet<String>, graph: &HashMap<String, HashSet<String>>) -> Vec<HashSet<String>> {
    if p.is_empty() && x.is_empty() {
        return vec![r];
    }

    let mut cliques = vec![];

    for v in p.clone() {
        let mut nr = r.clone();
        nr.insert(v.clone());
        let np = p.intersection(&graph[v.as_str()]).cloned().collect::<HashSet<_>>();
        let nx = x.intersection(&graph[v.as_str()]).cloned().collect::<HashSet<_>>();
        cliques.append(&mut bron_kerbosch(nr, np, nx, graph));
        p.remove(&v);
        x.insert(v.clone());
    }

    cliques
}

pub(crate) fn part2(input: String) {
    let graph = parse(&input);
    let p = graph.keys().cloned().collect::<HashSet<_>>();
    let cliques = bron_kerbosch(HashSet::new(), p, HashSet::new(), &graph);
    let mut max = cliques
        .into_iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    max.sort_unstable();
    println!("{}", max.join(","));
}
