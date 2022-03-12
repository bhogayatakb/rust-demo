// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
// use List::{Cons, Nil};

/* pub fn run() {
    let x = 1;
    let x_box: Box<i32> = Box::new(x);
    println!("{:?}", (x, x_box));
    
    // let list1: List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // let list2: List = Cons(1, Box::new(Nil));
    
    let y = Box::new(5);
    println!("y --- {:?}", y);
    println!("&y --- {:?}", &y);
} */

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy_head = ListNode::new(0);
    println!("dummyhead {:?}", dummy_head);
    let mut current = &mut dummy_head;
    println!("current {:?}", current);
    let mut p = l1;
    let mut q = l2;

    let mut carry: i32 = 0;
    println!("p {:?}", &p);
    println!("q {:?}", &q);

    while p != None || q != None {
        println!("&p {:?}", &p);
        println!("&q {:?}", &q);
        let sum = match (&p, &q) {
            (Some(l1), Some(l2)) => l1.val + l2.val + carry,
            (Some(l1), None) => l1.val + carry,
            (None, Some(l2)) => l2.val + carry,
            (None, None) => carry,
        };

        carry = sum / 10;
        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();

        p = if p != None { p.unwrap().next } else { p };
        q = if q != None { q.unwrap().next } else { q };
    }
    if carry > 0 {
        current.next = Some(Box::new(ListNode::new(carry)));
    }

    dummy_head.next
}