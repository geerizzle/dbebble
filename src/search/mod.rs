use std::{collections::HashMap, time::Instant};

use regex::Regex;

#[derive(Default)]
pub struct QGramIndex {
    index: HashMap<String, Vec<String>>,
}

impl QGramIndex {
    pub fn build(&mut self, stations: &HashMap<String, String>) {
        let start = Instant::now();
        println!("LOG: Building QGram index...");
        println!("LOG: Stations: {:?}", stations.len());
        for (eva, name) in stations.iter() {
            let qgrams = self.compute_qgrams(name, 2);
            for qgram in qgrams {
                self.index
                    .entry(qgram)
                    .or_insert(Vec::new())
                    .push(eva.clone());
            }
        }
        let duration = start.elapsed();
        println!("LOG: QGram index built in {:?}s", duration.as_secs());
    }

    pub fn print_index(&self) {
        self.index.iter().for_each(|(k, v)| {
            println!("{}: {:?}", k, v.len());
        });
    }

    pub fn get_index(&self) -> &HashMap<String, Vec<String>> {
        &self.index
    }

    pub fn normalize_query(&self, query: &str) -> String {
        let re = Regex::new(r"\w+").unwrap();
        let result = re
            .find_iter(query)
            .flat_map(|m| m.as_str().chars())
            .collect::<String>()
            .to_lowercase();
        result
    }

    pub fn search(&self, query: &str, stations: &HashMap<String, String>) -> Vec<String> {
        let query = query.to_lowercase();
        let qgrams = self.compute_qgrams(&query, 2);
        let results = Vec::new();
        results
    }

    fn compute_qgrams(&self, s: &String, q: usize) -> Vec<String> {
        let s = self.normalize_query(s);
        let s = format!("$${}", s);
        s.chars()
            .collect::<Vec<char>>()
            .windows(q)
            .map(|window| window.iter().collect())
            .collect()
    }

    fn ped(&self, s1: &str, s2: &str) -> usize {
        todo!("Implement PED")
    }
}

mod tests {
    #[test]
    fn test_compute_qgrams() {
        let search = super::QGramIndex::default();
        let s = "hello".to_string();
        let q = 2;
        let expected = vec!["$$", "$h", "he", "el", "ll", "lo"];
        let result = search.compute_qgrams(&s, q);
        assert_eq!(result, expected);
    }
}
