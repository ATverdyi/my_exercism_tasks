use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {
    let mut base_word: Vec<char> = word.to_lowercase().chars().collect();
    base_word.sort();
    
    let mut res = HashSet::new();
    
    for &w in possible_anagrams {
        let mut test_w: Vec<char> = w.to_lowercase().chars().collect();
        test_w.sort();

        if test_w == base_word && word.to_lowercase() != w.to_lowercase() {
            res.insert(w);
        }
    }
    res
}
