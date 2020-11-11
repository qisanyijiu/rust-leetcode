pub struct Solution {}

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let kb_rows = vec![
            vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
            vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'],
            vec!['z', 'x', 'c', 'v', 'b', 'n', 'm']];
        let mut kb_row_words = Vec::new();
        'l: for word in words {
            let mut last_i = 100;
            for c in word.chars() {
                for (i, kb_row) in kb_rows.iter().enumerate() {
                    if kb_row.contains(&c) {
                        if last_i == 100 {
                            last_i = i;
                        } else {
                            if last_i != i {
                                continue 'l;
                            }
                        }
                    }
                }
            }
            kb_row_words.push(word);
        }
        kb_row_words
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct Case {
        words: Vec<String>,
        result: Vec<String>
    } 
    
    #[test]
    fn test() {
        let cases = vec![
            Case{
                words: vec![String::from("Hello"), String::from("Alaska"), String::from("Dad"), String::from("Peace")],
                result: vec![String::from("Alaska"), String::from("Dad")]
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::find_words(case_item.words.clone());
            assert_eq!(result, case_item.result)
        }
    }
}