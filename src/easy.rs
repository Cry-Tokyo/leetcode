pub struct Solution;
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        word1
            .chars()
            .zip(word2.chars())
            .flat_map(|(x, y)| [x, y])
            .collect()
    }
}
