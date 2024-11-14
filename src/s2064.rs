// 2064. Minimized Maximum of Products Distributed to Any Store

// O(mlog max(quantities)) time, O(1) space
// Runtime: 18 ms, faster than 100%

// Better I saw is O(mlog max(quantities)) time, O(n) space
// At 14/11/24

pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
    let mut left = 1;
    let mut right = *quantities.iter().max().unwrap();

    // O(mlog_2 max(quantities))
    while left < right {
        let mid = (right + left) / 2;
        let mut stores = 0;

        //O(m)
        for q in quantities.iter() {
            stores += (q + mid - 1) / mid;
        };

        if stores <= n {
            right = mid;
        } else {
            left = mid + 1;
        };
    };

    left
}