use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use super::wordlist;

type SortedMap = HashMap<String, Vec<String>>;

// Implementation
pub struct Solver {
    anagrams: SortedMap,

}

impl Solver {
    pub async fn new() -> Self {
        if !Path::new("words.txt").exists() {
            wordlist::obtain_latest_wordlist().await.expect("Worldlist could not be obtained");
        }

        let anagrams: SortedMap = Self::sort_map();

        Self { anagrams }
    }

    pub fn display_anagram(&self, search_term:String) {
        let mut output_string:String = String::new();
        let mut cs: Vec<char> = search_term.chars().collect();
        cs.sort_by(|a,b| a.cmp(b));
        let ordered_search_term = String::from_iter(cs);

        let found_anagrams = self.anagrams.get(&ordered_search_term).unwrap();
        output_string += &format!("Anagrams for {} are ", search_term); 
        for anagram in found_anagrams {
            output_string += &format!("{}, ", anagram);
        }
        print!("{}\n", output_string);
    }

    pub fn display(&self) {
        let mut output_string:String = String::new();
        for (k,v) in self.anagrams.clone() {
            output_string += &format!("{} : ", k);
            for s in v {
                output_string += &format!("{} | ",s);
            }
            output_string += "\n";
        }
        print!("{}", output_string);
    }

    fn sort_map() -> SortedMap {
        let mut sorted_map: SortedMap = SortedMap::new();
        let file = File::open("words.txt").unwrap();
        let lines = io::BufReader::new(file).lines();

        for line in lines {
            let unwrapped: String = line.unwrap().to_lowercase();
            let mut cs: Vec<char> = unwrapped.chars().collect();
            cs.sort_by(|a,b| a.cmp(b));
            let ordered_variant: String = String::from_iter(cs);
            
            if !sorted_map.contains_key(&ordered_variant) {
                let mut temp: Vec<String> = Vec::new();
                temp.push(unwrapped);
                sorted_map.insert(ordered_variant, temp);
            } else {
                let temp = sorted_map.get_mut(&ordered_variant);
                let tu = temp.unwrap();
                tu.push(unwrapped);
            }
        }
        return sorted_map; 
    }
}
