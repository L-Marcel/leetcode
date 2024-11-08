// 1829. Maximum XOR for Each Query

// O(n) time, O(n) space
// Runtime: 0 ms, faster than 100%

// Better I saw is O(n) time, O(n) space
// At 08/11/24

pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::with_capacity(nums.len());
    let complement: i32 = (1 << maximum_bit) - 1;

    //O(n)
    let mut xor: i32 = nums.iter()
        .fold(0, |acc, x| acc ^ x);

    //O(n)
    for x in nums.iter().rev() {
        result.push(xor ^ complement);
        xor ^= x;
    }

    return result;
}