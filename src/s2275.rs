// 2275. Largest Combination With Bitwise AND Greater Than Zero

// O(n) time, O(1) space
// Runtime: 23 ms, faster than 100%

// Better I saw is O(n) time, O(1) space
// At 07/11/24

pub fn largest_combination(candidates: Vec<i32>) -> i32 {
    let mut bigger: i32 = 0;
    let mut counter: Vec<i32> = vec![0; 24];

    //O(n)
    candidates.iter().for_each(|x| {
        //O(24) = O(1)
        let mut size: u32 = x.ilog2() + 1;
        while size > 0 {
            if x & (1 << size - 1) != 0 {
                let pos: usize = (size - 1) as usize;
                counter[pos] += 1;
                if counter[pos] > bigger { bigger = counter[pos] };
            }
            size -= 1;
        }
    });
    
    return bigger;
}