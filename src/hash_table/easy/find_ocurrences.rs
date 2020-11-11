pub struct Solution {}

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let strings = text.split_whitespace().into_iter().collect::<Vec<&str>>();
        if strings.len() < 3 {
            return result
        }
        let mut start_index = 0;
        while start_index < strings.len() - 2 {
            if strings[start_index].to_string() == first && strings[start_index+1].to_string() == second{
                result.push(strings[start_index+2].to_string());
                start_index += 2;
            }else{
                start_index += 1
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        text: String,
        first: String,
        second: String,
        result: Vec<String>
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                text: "alice is a good girl she is a good student".to_string(),
                first: "a".to_string(),
                second: "good".to_string(),
                result: vec!["girl".to_string(),"student".to_string()],
            },
            Case{
                text: "we will we will rock you".to_string(),
                first: "we".to_string(),
                second: "will".to_string(),
                result: vec!["we".to_string(),"rock".to_string()]
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::find_ocurrences(case_item.text.clone(), case_item.first.clone(), case_item.second.clone());
            assert_eq!(result, case_item.result)
        }
    }
}