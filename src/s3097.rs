// 3097. Shortest Subarray With OR at Least K II

// O(n) time, O(1) space
// Runtime: 24 ms, faster than 100%

// Better I saw is O(n) time, O(1) space
// At 10/11/24

pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut min: i32 = -1;
    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut bits = [0; 32];

    //O(2n) = O(n) -> because the second while executes 
    // at most n times inside the first while
    while end < nums.len() {
        let x = nums[end];
        for i in 0..32 {
            if (x >> i) & 1 == 1 {
                bits[i] += 1;
            };
        };

        let mut current = 0;
        for i in 0..32 {
            if bits[i] != 0 {
                current |= 1 << i;
            };
        };

        while start <= end && current >= k {
            let size = (end - start + 1) as i32;
            if size < min || min == -1 { min = size };

            let y = nums[start];
            for i in 0..32 {
                if (y >> i) & 1 == 1 {
                    bits[i] -= 1;
                };
            };

            current = 0;
            for i in 0..32 {
                if bits[i] != 0 {
                    current |= 1 << i;
                };
            };

            start += 1;
        };

        end += 1;
    };

    min
}