use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    // todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let word_sorted_lowercase = sort_lowercase(word);
    possible_anagrams
        .into_iter()
        .fold(HashSet::new(), |mut acc, &w| {
            if word.to_lowercase() != w.to_lowercase() && sort_lowercase(w) == word_sorted_lowercase
            {
                acc.insert(w);
            }
            acc
        })
}
/**
 * sort and to_lowercase
 */
fn sort_lowercase(s: &str) -> String {
    let mut chars = s.to_lowercase().chars().collect::<Vec<char>>();
    chars.sort();
    chars.into_iter().collect::<String>()
}
