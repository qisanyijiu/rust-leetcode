use std::thread::sleep;

pub struct NumArray{
    sum: Vec<i32>
}

impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let mut sum:Vec<i32> = Vec::new();
        for (i, num) in nums.into_iter().enumerate() {
            if i == 0 {
                sum.push(num)
            } else {
                sum.push(sum[i-1] + num)
            }
        }
        NumArray {
            sum
        }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        let right = self.sum[j as usize];
        if i > 0 {
            right - self.sum[(i-1) as usize]
        }else{
            right
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        i: i32,
        j: i32,
        result: i32
    }

    #[test]
    fn test() {
        let nums: Vec<i32> = vec![-2, 0, 3, -5, 2, -1];
        let cases = vec![
            Case{
                i: 0,
                j: 2,
                result: 1
            },
            Case{
                i: 2,
                j: 5,
                result: -1
            },
            Case{
                i: 0,
                j: 5,
                result: -3
            }
        ];
        let numArray = NumArray::new(nums.clone());
        for case_item in cases.iter(){
            let result = numArray.sum_range(case_item.i, case_item.j);
            assert_eq!(result, case_item.result)
        }
    }
}