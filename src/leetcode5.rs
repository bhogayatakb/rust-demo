// longest palindrome substring

pub fn longest_palindrome(s: String) -> String {
    let (s, mut max) = (s.chars().collect::<Vec<char>>(), vec![]);
    println!("std::usize::MIN ==> {:?}", std::usize::MIN);
    fn find_max(s: &Vec<char>, max: Vec<char>, i: usize, j: usize) -> Vec<char> {
        let (mut i, mut j) = (i, j);
        let mut sub: &[char] = &[];

       

        while i != std::usize::MAX && j < s.len() && s[i] == s[j] {
            sub = &s[i..j+1];

            println!("i ==> {:?}", i);
            println!("j ==> {:?}", j);
            println!("---------------------");

            if i == std::usize::MIN {
                break;
            } else {
                i -= 1;
                j += 1;
            }                
        }
        if sub.len() > max.len() {
            return sub.to_vec()
        }
        max.to_vec()
    }
    for i in 0..s.len() {
        max = find_max(&s, max, i, i);
        max = find_max(&s, max, i, i+1);
    }
    max.into_iter().collect()
}