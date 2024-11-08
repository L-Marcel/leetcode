mod s2914;
mod s3011;
mod s2275;
mod s1829;

fn main() {
    assert_eq!(s2914::min_changes("0100".to_string()), 1);
    assert_eq!(s2914::min_changes("0101".to_string()), 2);
    assert_eq!(s3011::can_sort_array([8,4,2,30,15].to_vec()), true);
    assert_eq!(s2275::largest_combination([16,17,71,62,12,24,14].to_vec()), 4);
    assert_eq!(s2275::largest_combination([8,8].to_vec()), 2);
    assert_eq!(s1829::get_maximum_xor([0,1,1,3].to_vec(), 2), [0,3,2,3].to_vec());
    assert_eq!(s1829::get_maximum_xor([2,3,4,7].to_vec(), 3), [5,2,6,5].to_vec());
}
