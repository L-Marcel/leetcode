// 2914. Minimum Number of Changes to Make Binary String Beautiful

// O(n) time, O(1) space
// Runtime: 0 ms, faster than 100.00%

// Better I saw is O(n) time, O(1) space
// At 05/11/24

pub fn min_changes(s: String) -> i32 {
    //O(n) + O(n) = O(n)
    s.as_bytes().chunks(2).map(|chunk: &[u8]| {
        match chunk {
            [a, b] if a == b => 0,
            _ => 1
        }
    }).sum::<i32>()
}