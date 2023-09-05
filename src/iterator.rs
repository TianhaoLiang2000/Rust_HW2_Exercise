pub fn iterator() -> Vec<char> {
    let a: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
    let iter = a.iter().map(|&x| (x as u8 + 1) as char);
    let result: Vec<char> = iter.collect();
    result
}