// 2070. Most Beautiful Item for Each Query

// O((n + m)log n) time, O(m) space
// Runtime: 24 ms, faster than 50%

// Better I saw is O((n + m)log n) time, O(n + m) space
// At 12/11/24

//O(nlog n + mlog n) = O((n + m)log n)
pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    //O(nlog_2 n)
    items.sort_unstable();
    
    //O(n)
    let mut bigger = 0;
    for item in items.iter_mut() {
        if item[1] > bigger {
            bigger = item[1];
        } else {
            item[1] = bigger;
        };
    };

    //O(m * log_2 n)
    queries.iter().map(|query| {
        let best_position = items.partition_point(|item| {
            item[0] <= *query
        });

        match  best_position {
            0 => 0,
            _ => items[best_position - 1][1]
        }
    }).collect()
}