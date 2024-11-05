// 2914. Minimum Number of Changes to Make Binary String Beautiful

// O(n) time, O(1) space
// Runtime: 0 ms, faster than 100.00%

#[allow(dead_code)]
pub fn min_changes(s: String) -> i32 {
    s.as_bytes().chunks(2).map(|chunk: &[u8]| {
        match chunk {
            [a, b] if a == b => 1,
            _ => 0
        }
    }).sum::<i32>()
}