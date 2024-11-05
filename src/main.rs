mod s2914;

fn main() {
    assert_eq!(s2914::min_changes("0100".to_string()), 1);
    assert_eq!(s2914::min_changes("0101".to_string()), 2);
}
