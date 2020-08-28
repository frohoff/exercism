use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    fn get_freq(word: &str) -> HashMap<char, u32> {
        let mut freq = HashMap::new();
        for c in word.chars() {
            *freq.entry(c).or_insert(0) += 1;
        }
        freq
    }

    let l_word = word.to_lowercase();    
    let word_freq = get_freq(&l_word);
    possible_anagrams.iter()
        .filter(|a| { 
            let l_a = a.to_lowercase(); 
            get_freq(&l_a) == word_freq && l_word != l_a 
        })
        .map(|&a| a)
        .collect()
}