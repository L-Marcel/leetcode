// 3133. Minimum Array End

// O(1) time, O(1) space
// Runtime: 0 ms, faster than 100%

// Better I saw is O(1) time, O(1) space
// At 09/11/24

pub fn min_end(n: i32, x: i32) -> i64 {
    let size = 32 - x.leading_zeros();
    let spaces = (((1 << size) - 1) - x).count_ones() as i32;
    let combinations = 1 << spaces;
    let rest = (n - 1) % combinations;
    let border = ((n - 1) / combinations) as i64;
    let left: i64 = (border << size) as i64;

    //O(32) = O(1)
    let mut bit = 1;
    let mut rest_bit = 1_i32;
    let mut right = x;
    while bit <= (1 << (size - 1)) {
        if right & bit == 0 {
            if rest_bit & rest != 0 {
                right |= bit;
            }
            rest_bit = rest_bit << 1;
        };
        bit = bit << 1;
    }

    left | (right as i64)
}