pub struct Solution {}

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let s_bytes = secret.as_bytes();
        let g_bytes = guess.as_bytes();

        let mut bull = 0;
        let mut cow = 0;

        let mut tmp: [i32; 10] = [0; 10];
        for i in 0..secret.len() {
            let s = s_bytes[i] as usize - '0' as usize;
            let g = g_bytes[i] as usize - '0' as usize;
            if s == g {
                bull += 1;
                continue;
            }
            if tmp[s] < 0 {
                cow += 1
            }
            if tmp[g] > 0 {
                cow += 1
            }
            tmp[s] = tmp[s] + 1;
            tmp[g] = tmp[g] - 1;
        }
        let result = format!("{}A{}B", bull, cow);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        secret: String,
        guess: String,
        result: String,
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                secret: "1807".to_string(),
                guess: "7810".to_string(),
                result: "1A3B".to_string(),
            },
            Case {
                secret: "1123".to_string(),
                guess: "0111".to_string(),
                result: "1A1B".to_string(),
            }
        ];
        for case_item in cases.iter() {
            let result = Solution::get_hint(case_item.secret.clone(), case_item.guess.clone());
            assert_eq!(result, case_item.result)
        }
    }
}