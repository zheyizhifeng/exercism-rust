extern crate unicode_reverse;
use unicode_reverse::reverse_grapheme_clusters_in_place;

pub fn reverse(input: &str) -> String {
    let mut input = input.to_owned();
    reverse_grapheme_clusters_in_place(&mut input);
    input.to_string()
}

// pub fn reverse(input: &str) -> String {
//     input.chars().rev().collect()
// }

