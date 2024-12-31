use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
fn main() {
    let path = "./src/data.txt";
    let file = File::open(path).expect("Error opening file");

    let duos = get_input_from_file(&file);

    // first star
    let triples = find_3_sets(duos.clone());
    let answer = triples
        .iter()
        .filter(|triple| {
            triple.0.starts_with("t") || triple.1.starts_with("t") || triple.2.starts_with("t")
        })
        .count();
    println!("First star answer : {:?}", answer);

    // Second star
    let answer = find_password(triples, duos);
    println!("Second star answer : {:?}", answer);
}

fn find_3_sets(input: HashSet<(String, String)>) -> Vec<(String, String, String)> {
    let mut result = Vec::new();
    for a in &input {
        for b in &input {
            if a.0 == b.1
                && (input.contains(&(a.1.clone(), b.0.clone()))
                    || input.contains(&(b.0.clone(), a.1.clone())))
            {
                if result.iter().any(|(x, y, z): &(String, String, String)| {
                    (x == &a.0 && y == &a.1 && z == &b.0)
                        || (x == &a.0 && y == &b.0 && z == &a.1)
                        || (x == &a.1 && y == &a.0 && z == &b.0)
                        || (x == &a.1 && y == &b.0 && z == &a.0)
                        || (x == &b.0 && y == &a.0 && z == &a.1)
                        || (x == &b.0 && y == &a.1 && z == &a.0)
                }) {
                    continue;
                }
                result.push((a.0.clone(), a.1.clone(), b.0.clone()));
            }
        }
    }
    result
}

fn find_password(
    triples: Vec<(String, String, String)>,
    input: HashSet<(String, String)>,
) -> String {
    let mut combinaisons: HashMap<String, HashSet<String>> = HashMap::new();
    for (a, b) in input.iter() {
        combinaisons.entry(a.clone()).or_default().insert(b.clone());
        combinaisons.entry(b.clone()).or_default().insert(a.clone());
    }
    let mut max_networks = HashSet::new();

    for triple in triples {
        let mut network = HashSet::new();
        network.insert(triple.0);
        network.insert(triple.1);
        network.insert(triple.2);

        for element in network.clone() {
            if let Some(connected_elements) = combinaisons.get(&element) {
                for connected in connected_elements {
                    if let Some(next_level_connections) = combinaisons.get(connected) {
                        if network.is_subset(next_level_connections) {
                            network.insert(connected.clone());
                        }
                    }
                }
            }
        }

        if network.len() > max_networks.len() {
            max_networks = network;
        }
    }

    let mut max_network_vec: Vec<String> = max_networks.iter().cloned().collect();
    max_network_vec.sort();

    max_network_vec.join(",")
}

fn get_input_from_file(file: &File) -> HashSet<(String, String)> {
    let mut duos: HashSet<(String, String)> = HashSet::new();
    let reader: io::BufReader<&File> = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let duo: Vec<&str> = line.split("-").collect();
        duos.insert((duo[0].to_string(), duo[1].to_string()));
    }
    duos
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first() {
        let input: HashSet<(String, String)> = [
            ("kh", "tc"),
            ("qp", "kh"),
            ("de", "cg"),
            ("ka", "co"),
            ("yn", "aq"),
            ("qp", "ub"),
            ("cg", "tb"),
            ("vc", "aq"),
            ("tb", "ka"),
            ("wh", "tc"),
            ("yn", "cg"),
            ("kh", "ub"),
            ("ta", "co"),
            ("de", "co"),
            ("tc", "td"),
            ("tb", "wq"),
            ("wh", "td"),
            ("ta", "ka"),
            ("td", "qp"),
            ("aq", "cg"),
            ("wq", "ub"),
            ("ub", "vc"),
            ("de", "ta"),
            ("wq", "aq"),
            ("wq", "vc"),
            ("wh", "yn"),
            ("ka", "de"),
            ("kh", "ta"),
            ("co", "tc"),
            ("wh", "qp"),
            ("tb", "vc"),
            ("td", "yn"),
        ]
        .iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect();

        let result = find_3_sets(input);
        assert_eq!(result.len(), 12);
        let answer = result
            .iter()
            .filter(|triple| {
                triple.0.starts_with("t") || triple.1.starts_with("t") || triple.2.starts_with("t")
            })
            .count();
        assert_eq!(answer, 7);
    }

    #[test]
    fn test_second() {
        let input: HashSet<(String, String)> = [
            ("ka", "co"),
            ("ta", "co"),
            ("de", "co"),
            ("ta", "ka"),
            ("de", "ta"),
            ("ka", "de"),
        ]
        .iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect();

        let triples: Vec<(String, String, String)> = find_3_sets(input.clone());
        let password = find_password(triples, input);

        assert_eq!(password, "co,de,ka,ta");
    }
}
