use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    fn sorted(s: &str) -> String {
        let mut sorted: Vec<_> = s.chars().collect();
        sorted.sort();
        sorted.iter().collect::<String>()
    }
    let l_word = word.to_lowercase();
    let s_word = sorted(&l_word);
    
    possible_anagrams.iter()
        .filter(|a| { 
            let l_a = a.to_lowercase(); 
            sorted(&l_a) == s_word && l_word != l_a 
        })
        .map(|&a| a)
        .collect()
}