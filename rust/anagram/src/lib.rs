use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    println!("check {:?} {:?}", word, possible_anagrams);
    pub fn get_freq(word: &str) -> HashMap<char, u32> {
        let mut freq = HashMap::new();
        for c in word.to_lowercase().chars() {
            *freq.entry(c).or_insert(0) += 1;
        }
        println!("freq {:?} {:?}", word, freq);
        freq
        
    }
    
    let word_freq = get_freq(word);
    possible_anagrams.iter().filter(|a| get_freq(a) == word_freq ).map(|a| *a).collect()
}