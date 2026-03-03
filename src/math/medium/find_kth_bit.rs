pub struct Solution {}

impl Solution {
    pub fn find_kth_bit(n: i32, mut k: i32) -> char {
        /**
         * S0 = 0
         * S1 = 0 + 1 + 1
         * S2 = 011 + 1 + 001
         * S3 = 0111001 + 1 + 0110001
         * ...
         * Sn = Sn-1 + 1 + reverse(invert(Sn-1))
         * 
         * 
         */
        let mut invert_count = 0;
        let mut len = (1 << n) - 1;
        while (k > 1)  {
            if k == len/2 + 1 {
                if invert_count % 2 == 0 {
                    return '1';
                } else {
                    return '0';
                }
            }
            if k > len/2 {
                k = (len - k + 1) as i32;
                invert_count+=1;
            }
            len /= 2;
        }
        if invert_count % 2 == 0 {
            return '0';
        } else {
            return '1';
        }
    }
}