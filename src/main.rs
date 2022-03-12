// mod print;
// mod vars;
// mod types;
// mod leetcode2;
// mod leetcode3;
mod leetcode5;


fn main() {
    // print::run();
    // vars::run();
    // types::run()
    // leetcode2::run();

    /* 
    // --- Leetcode 2 : Start ---

    let mut listnode1 = leetcode2::ListNode::new(2);
    let mut listnode11 = leetcode2::ListNode::new(4);
    let listnode111 = leetcode2::ListNode::new(3);

    listnode11.next = Some(Box::new(listnode111));
    listnode1.next =  Some(Box::new(listnode11));

    let listnode2 = leetcode2::ListNode::new(5);

    let boxx1 = Box::new(listnode1);
    let boxx2 = Box::new(listnode2);

    println!("check ===> {:?}", boxx1.val);

    let param1: Option<Box<leetcode2::ListNode>> = Some(boxx1);
    let param2: Option<Box<leetcode2::ListNode>> = Some(boxx2);

    println!("Result ===> {:?}", leetcode2::add_two_numbers(param1, param2));

    // --- Leetcode 2 : End ---
    */
    
    /* 
    // --- Leetcode 3 : Start ---
    println!("result {:?}",leetcode3::length_of_longest_substring(String::from("abcdeeefghijk")));
    // --- Leetcode 3 : End ---
    */

    
    // --- Leetcode 5 : Start ---
    println!("result {:?}",leetcode5::longest_palindrome(String::from("abcdedcbauyyy")));
    // --- Leetcode 5 : End ---
   

   
}

/* pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    
    let mut m: HashMap<i32, i32> = HashMap::new();
    for (i, v) in nums.iter().enumerate() {

        
        println!("&(target - *v) {:?}", m.get(&(target - *v)));
        match m.get(&(target - *v)) {
            Some(&i2) => return vec![i as i32, i2],
            None => m.insert(*v, i as i32),
        };
        println!("m {:?}", m);
    }
    vec![]
} */

/* fn give_three() -> i32 {
    3
} */

#[cfg(test)]  // tag to let compiler know what not to compile on "cargo run"
mod test_demo {
    #[test] // tag to let compiler know what to run on "cargo test"
    fn test_basic() {
        assert!(1 == 1)
    }

    #[test] // tag to let compiler know what to run on "cargo test"
    #[should_panic] // shows that test should panic / fail
    fn test_basic2() {
        assert!(1 == 2)
    }

    #[test] // tag to let compiler know what to run on "cargo test"
    fn test_basic3() {
        assert!(super::give_three() == 3)
    }
}
