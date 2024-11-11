// 796. Rotate String

// O(n^2) time, O(1) space
// Runtime: 0 ms, faster than 100%

// Better I saw is O(n^2) time, O(1) space
// At 08/11/24

pub fn rotate_string(s: String, goal: String) -> bool {
    if s.len() != goal.len() {
        return false;
    }; 

    // O(2n + n^2) = O(n^2)
    s.repeat(2).contains(&goal)
}