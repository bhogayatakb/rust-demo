// longest substring without repeating characters

use std::collections::HashMap;
use std::cmp;

pub fn length_of_longest_substring(s: String) -> i32 {
	let mut len = 0;
	let mut j = 0;
	let mut map = HashMap::new();
	for (i, c) in s.char_indices() {
        println!("c ==> {:?}", c);
            println!("i+1 ==> {:?}", i + 1);
		if let Some(last) = map.insert(c, i + 1) {
            println!("last ==> {:?}", last);
			j = cmp::max(j, last)
		}
        println!("i - j + 1 ==> {:?}", i - j + 1);
		len = cmp::max(len, i - j + 1);
	}
	len as i32
}