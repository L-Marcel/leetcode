// 1574. Shortest Subarray to be Removed to Make Array Sorted

pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
    let mut left: usize = 0;
    while left < arr.len() - 1 && arr[left] <= arr[left + 1] {
        left += 1;
    };

    if left == arr.len() - 1 { return 0; };

    let mut right: usize = arr.len() - 1;
    while right > 0 && arr[right - 1] <= arr[right] {
        right -= 1;
    };

    let mut size: i32 = right as i32;
    let left_size: i32 = (arr.len() - left - 1) as i32;
    if left_size < size {
        size = left_size;
    };

    let mut answer: i32 = size;
    let mut index: usize = 0;
    while index <= left && right < arr.len() {
        if arr[index] <= arr[right] {
            size = (right - index - 1) as i32;
            if size < answer {
                answer = size;
            };
            index += 1;
        } else {
            right += 1;
        };
    };

    answer
}