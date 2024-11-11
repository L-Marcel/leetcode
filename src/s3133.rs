// 3133. Minimum Array End

// O(1) time, O(1) space
// Runtime: 0 ms, faster than 100%

// Better I saw is O(1) time, O(1) space
// At 09/11/24

pub fn min_end(n: i32, x: i32) -> i64 {
    let size: u32 = 32 - x.leading_zeros();
    let spaces: i32 = (((1 << size) - 1) - x).count_ones() as i32;
    let combinations: i32 = 1 << spaces;
    let rest: i32 = (n - 1) % combinations;
    let border: i64 = ((n - 1) / combinations) as i64;
    let left: i64 = (border << size) as i64;

    //O(32) = O(1)
    let mut bit: i32 = 1;
    let mut rest_bit: i32 = 1;
    let mut right: i32 = x;
    let max_bit: i32 = 1 << (size - 1);
    while bit <= max_bit {
        if right & bit == 0 {
            if rest_bit & rest != 0 {
                right |= bit;
            };
            rest_bit = rest_bit << 1;
        };
        bit = bit << 1;
    };

    left | (right as i64)
}