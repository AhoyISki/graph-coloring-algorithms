//! Some graph coloring algorithms
//!
//! In order to run these, just install [rust], and, from the cloned
//! repository's path, just call:
//!
//! ```bash
//! cargo run --release
//! ```
//! 
//! This program will read any file with a name like "*.col" in the
//! directory. These files must be formatted like this:
//!
//! ```text
//! c This is a comment line, there can be anything in here
//! c No line can be empty, apart from the last one
//! c
//! p edge {vertices} {edges}
//! e {vertex} {vertex}
//! ```
//!
//! The vertices should also be 1 indexed, for example:
//!
//! ```text
//! c This is a comment line, there can be anything in here
//! c No line can be empty, apart from the last one
//! c
//! p edge 3 2
//! e 1 3
//! e 1 2
//! ```
//!
//! [rust]: https://www.rust-lang.org/tools/install
use std::{collections::HashSet, time::Instant};

fn main() {
    verify_gca(first_fit, "first fit");
    verify_gca(welsh_powell, "welsh powell");
    verify_gca(largest_degree_ordering, "largest degree ordering");
    verify_gca(incidence_degree_ordering, "incidence degree ordering");
    verify_gca(degree_of_saturation, "degree of saturation");
    verify_gca(recursive_largest_first, "recursive largest first");
}

fn verify_gca(gca: GraphColoringAlgorithm, gca_name: &str) {
    let mut paper_test_colors = Vec::new();
    println!("{gca_name}:");

    for file in std::fs::read_dir(".").unwrap().filter_map(Result::ok) {
        let name = file.file_name().into_string().unwrap();
        if name.ends_with(".col") {
            let edges = get_edges(&name);

            let start = Instant::now();
            let (v_colors, color) = gca(&edges);

            check_colors(&edges, &v_colors);

            if name == "paper-test.col" {
                paper_test_colors = v_colors;
            }

            println!(
                "  took {:.2?} on {name} with {color:?} colors",
                start.elapsed()
            );
        }
    }

    println!("\n  paper test colors: {paper_test_colors:?}\n");
}

////////// Sorting algorithms

fn first_fit(edges: &[Vec<bool>]) -> (Vec<usize>, usize) {
    let mut color = 0;
    let mut v_colors = vec![usize::MAX; edges.len()];

    for i_v in 0..edges.len() {
        assign_color(i_v, &mut v_colors, edges, &mut color);
    }

    (v_colors, color)
}

fn welsh_powell(edges: &[Vec<bool>]) -> (Vec<usize>, usize) {
    let mut color = 0;
    let mut v_colors = vec![usize::MAX; edges.len()];
    let degrees = degree_list(edges);
    let mut v_is = sorted_index_list(edges, &degrees);

    while let Some(i_vmax) = v_is.pop() {
        v_colors[i_vmax] = color;

        let mut not_adj: Vec<usize> = (0..v_is.len())
            .filter(|i_i| !edges[i_vmax][v_is[*i_i]])
            .collect();

        while let Some(i_i) = not_adj.pop() {
            let i_v = v_is.remove(i_i);
            v_colors[i_v] = color;
            not_adj.retain(|i_i| !edges[i_v][v_is[*i_i]])
        }

        color += 1;
    }

    (v_colors, color)
}

fn largest_degree_ordering(edges: &[Vec<bool>]) -> (Vec<usize>, usize) {
    let mut color = 0;
    let mut v_colors = vec![usize::MAX; edges.len()];
    let mut v_is = sorted_index_list(edges, &degree_list(edges));

    while let Some(i_v) = v_is.pop() {
        assign_color(i_v, &mut v_colors, edges, &mut color);
    }

    (v_colors, color)
}

fn incidence_degree_ordering(edges: &[Vec<bool>]) -> (Vec<usize>, usize) {
    let mut color = 1;
    let mut v_colors = vec![usize::MAX; edges.len()];
    let mut v_is = sorted_index_list(edges, &degree_list(edges));
    let mut colored_adjs = vec![0; edges.len()];

    let i_vmax = v_is.pop().unwrap();
    v_colors[i_vmax] = 0;
    for (j_v, adj) in edges[i_vmax].iter().enumerate() {
        colored_adjs[j_v] = *adj as usize;
    }

    while !v_is.is_empty() {
        let i_imax = (0..v_is.len())
            .max_by_key(|i_i| (colored_adjs[v_is[*i_i]], *i_i))
            .unwrap();
        let i_v = v_is.remove(i_imax);

        for (j_v, adj) in edges[i_v].iter().enumerate() {
            colored_adjs[j_v] += *adj as usize;
        }

        assign_color(i_v, &mut v_colors, edges, &mut color);
    }

    (v_colors, color)
}

fn degree_of_saturation(edges: &[Vec<bool>]) -> (Vec<usize>, usize) {
    let mut color = 1;
    let mut v_colors = vec![usize::MAX; edges.len()];
    let mut v_is = sorted_index_list(edges, &degree_list(edges));
    let mut diff_adjs = vec![Vec::new(); edges.len()];

    let i_vmax = v_is.pop().unwrap();
    v_colors[i_vmax] = 0;
    for (j_v, _) in edges[i_vmax].iter().enumerate().filter(|(_, adj)| **adj) {
        diff_adjs[j_v].push(0);
    }

    while !v_is.is_empty() {
        let i_imax = (0..v_is.len())
            .max_by_key(|i_i| (diff_adjs[v_is[*i_i]].len(), *i_i))
            .unwrap();
        let i_vmax = v_is.remove(i_imax);

        assign_color(i_vmax, &mut v_colors, edges, &mut color);

        for (j_v, _) in edges[i_vmax].iter().enumerate().filter(|(_, adj)| **adj) {
            if !diff_adjs[j_v].contains(&v_colors[i_vmax]) {
                diff_adjs[j_v].push(v_colors[i_vmax]);
            }
        }
    }

    (v_colors, color)
}

fn recursive_largest_first(edges: &[Vec<bool>]) -> (Vec<usize>, usize) {
    let mut color = 1;
    let mut v_colors = vec![usize::MAX; edges.len()];
    let mut v_is = sorted_index_list(edges, &degree_list(edges));

    let mut i_vmax_opt = Some(v_is.pop().unwrap());
    v_colors[i_vmax_opt.unwrap()] = 0;

    while let Some(i_vmax) = i_vmax_opt {
        let mut adj: Vec<usize> = (0..edges.len()).filter(|j_v| edges[i_vmax][*j_v]).collect();
        let mut not_adj: Vec<usize> = (0..v_is.len())
            .filter(|j_i| !edges[i_vmax][v_is[*j_i]])
            .collect();

        while !not_adj.is_empty() {
            let i_not_adjmax = (0..not_adj.len())
                .max_by_key(|i_i| (0..adj.len()).filter(|j_v| edges[v_is[*i_i]][*j_v]).count())
                .unwrap();

            let i_i = not_adj.remove(i_not_adjmax);
            let i_v = v_is[i_i];

            adj.extend(not_adj.extract_if(.., |j_i| edges[i_v][v_is[*j_i]]));
            adj.push(i_v);

            v_colors[i_v] = color - 1;
        }

        v_colors[i_vmax] = color - 1;
        v_is.retain(|i_v| v_colors[*i_v] == usize::MAX);

        color += 1;
        i_vmax_opt = (0..v_is.len())
            .max_by_key(|i_i| {
                let adj = (0..v_is.len())
                    .filter(|j_i| edges[v_is[*i_i]][v_is[*j_i]])
                    .count();
                (adj, *i_i)
            })
            .map(|i_i| v_is[i_i]);
    }

    (v_colors, color)
}

////////// Common functions

fn degree_list(edges: &[Vec<bool>]) -> Vec<usize> {
    edges
        .iter()
        .map(|adjs| adjs.iter().filter(|adj| **adj).count())
        .collect()
}

fn sorted_index_list(edges: &[Vec<bool>], degrees: &[usize]) -> Vec<usize> {
    let mut v_indices: Vec<usize> = (0..edges.len()).collect();
    v_indices.sort_unstable_by_key(|i_v| degrees[*i_v]);
    v_indices
}

fn assign_color(i_v: usize, v_colors: &mut [usize], edges: &[Vec<bool>], color: &mut usize) {
    let taken: HashSet<usize> = HashSet::from_iter(
        v_colors
            .iter()
            .enumerate()
            .filter_map(|(j_v, c)| (*c != usize::MAX && edges[i_v][j_v]).then_some(*c)),
    );

    if let Some(c) = (0..*color).find(|c| !taken.contains(c)) {
        v_colors[i_v] = c;
    } else {
        v_colors[i_v] = *color;
        *color += 1;
    }
}

////////// Utility functions

fn get_edges(file: &str) -> Vec<Vec<bool>> {
    let file = std::fs::read_to_string(file).unwrap();
    let mut lines = file
        .lines()
        .enumerate()
        .skip_while(|(_, l)| l.starts_with("c"));

    let vertices: usize = {
        let (_, info_line) = lines.next().unwrap();
        info_line
            .split(" ")
            .nth(2)
            .unwrap_or_else(|| panic!("{info_line}"))
            .parse()
            .unwrap()
    };

    let mut edges = vec![vec![false; vertices]; vertices];

    for (line_i, line) in lines {
        let mut words = line.split(" ");

        // In case of an empty line.
        let Some(i): Option<usize> = words.nth(1).map(|word| word.parse().unwrap()) else {
            continue;
        };

        let j: usize = words
            .next()
            .unwrap_or_else(|| panic!("{line_i}: {line}"))
            .parse()
            .unwrap();
        edges[i - 1][j - 1] = true;
        edges[j - 1][i - 1] = true;
    }

    edges
}

#[track_caller]
fn check_colors(edges: &[Vec<bool>], v_colors: &[usize]) {
    assert!(
        edges
            .iter()
            .enumerate()
            .flat_map(|(i, e)| e.iter().enumerate().map(move |(j, adj)| (i, j, *adj)))
            .all(|(i, j, adj)| !adj || v_colors[i] != v_colors[j])
            && v_colors.iter().all(|c| *c != usize::MAX)
    )
}

type GraphColoringAlgorithm = fn(&[Vec<bool>]) -> (Vec<usize>, usize);
