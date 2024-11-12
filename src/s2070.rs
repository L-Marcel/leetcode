// 2070. Most Beautiful Item for Each Query

use std::collections::HashMap;

pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut queries_map: HashMap<i32, i32> = HashMap::with_capacity(queries.len());
    let mut answers: Vec<i32> = vec![0; queries.len()];
   
    for (index, query) in queries.iter().enumerate() {
        let mapped_queries = queries_map.get(query);
        match mapped_queries {
            None => {
                let mut best = -1;
                for item in items.iter() {
                    let price = item[0];
                    let beauty = item[1];
                    if price <= *query && (beauty > best || best == -1) {
                        best = beauty;
                    };
                };

                if best == -1 { best = 0; };
                queries_map.insert(*query, best);
                answers[index] = best;
            },
            Some(best) => {
                answers[index] = *best;
            }
        };
    };

    answers
}