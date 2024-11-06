// 3011. Find if Array Can Be Sorted

// O(n^2) time, O(1) space
// Runtime: 0 ms, faster than 100%

// Better I saw is O(n) time, O(1) space
// At 06/11/24

#[allow(dead_code)]
pub fn can_sort_array(nums: Vec<i32>) -> bool {
    let mut i = 1;
    for x in nums.iter().skip(1) {
        for y in nums.iter().take(i).rev() {
            if x < y  && x.count_ones() != y.count_ones() {
                return false;
            }
        }
        i += 1;
    }

    true
}