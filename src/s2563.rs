// 2563. Count the Number of Fair Pairs

// O(nlog n) time, O(n) space
// Runtime: 19 ms, faster than 100%

// Better I saw is O(nlog n) time, O(n) space
// At 13/11/24

pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
    nums.sort_unstable();

    let mut answer: i64 = 0;
    for left in 0..nums.len() {
        let start = nums.partition_point(|&next| nums[left] + next < lower);
        let end = nums[start..].partition_point(|&previous| nums[left] + previous <= upper);

        answer += end as i64;
        if start <= left && left < start + end {
            answer -= 1;
        };
    };

    answer / 2
}