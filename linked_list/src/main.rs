use std::rc::Rc;
use std::clone::Clone;
use std::cell::RefCell;
// use core::mem::drop;

#[derive(Debug)]
struct ListNode {
    val : i64,
    prev : Option<Rc<RefCell<ListNode>>>,
    next : Option<Rc<RefCell<ListNode>>>
}

#[derive(Debug)]
struct List {
    count : i64,
    head : Option<Rc<RefCell<ListNode>>>,
    tail : Option<Rc<RefCell<ListNode>>>
}

impl ListNode {
    pub fn new(val: i64) -> Self {
        ListNode {
            val: val,
            prev: None,
            next: None
        }
    }
}

impl List {
    pub fn new() -> Self {
        let head = Rc::new(RefCell::new(ListNode::new(-1)));
        let tail = Rc::new(RefCell::new(ListNode::new(-1)));
        head.borrow_mut().next = Some(Rc::clone(&tail));
        tail.borrow_mut().prev = Some(Rc::clone(&head));
        List {
            count: 0,
            head: Some(head),
            tail: Some(tail)
        }
    }

    pub fn walk_list(&self) {
        let mut node_iter: Rc<RefCell<ListNode>> = Rc::clone(self.head.as_ref().unwrap());
        let count = self.count;
        print!("head");
        // println!("self.tail {:?}", self.tail.as_ref().unwrap().borrow().prev.as_ref().unwrap().borrow().val);
        for _i in 1..=count {
            let node = Rc::clone(&node_iter);
            //println!("[{}] {}", i, node.borrow().val);
            if node.borrow().next.is_some() {
                node_iter = Rc::clone(node.borrow().next.as_ref().unwrap());
                print!("->{}", node_iter.borrow().val);
            }
        }
        print!("\n");
    }

    pub fn at(&self, index : i64) -> i64 {
        if index > self.count {
            println!("No nodes");
            -1
        } else {
            let mut val : i64 = -1;
            let mut node_iter : Rc<RefCell<ListNode>> = Rc::clone(self.head.as_ref().unwrap());
            for _i in 1..=index {
                let node = Rc::clone(&node_iter);
                if node.borrow().next.is_some() {
                    node_iter = Rc::clone(node.borrow().next.as_ref().unwrap());
                    val = node_iter.borrow_mut().val;
                }
            }
            val
        }
    }

    pub fn insert_after(&mut self, index : i64, val : i64) {
        if index > self.count {
            println!("No nodes");
        } else {
            let new_node = Rc::new(RefCell::new(ListNode::new(val)));
            let mut node_iter : Rc<RefCell<ListNode>> = Rc::clone(self.head.as_ref().unwrap());
            for _i in 1..index {
                let node = Rc::clone(&node_iter);
                if node.borrow().next.is_some() {
                    node_iter = Rc::clone(node.borrow().next.as_ref().unwrap());
                }
            }

            // Inset after node iter
            if let Some(ref _next) = node_iter.borrow().next {
                _next.borrow_mut().prev = Some(Rc::clone(&new_node));
                new_node.borrow_mut().next = Some(Rc::clone(_next));
            }

            node_iter.borrow_mut().next = Some(Rc::clone(&new_node));
            new_node.borrow_mut().prev = Some(Rc::clone(&node_iter));
            self.count += 1;
        }
    }

    pub fn take_out(&mut self, val : i64) -> bool {
        if self.count == 0 {
            println!("No nodes!");
            false
        } else {
            let mut find = false;
            let mut tmp : Option<Rc<RefCell<ListNode>>>= None;
            let mut node_iter : Rc<RefCell<ListNode>> = Rc::clone(self.head.as_ref().unwrap());
            for _i in 1..=self.count {
                if node_iter.borrow().val == val {
                    if let Some(ref _next) = node_iter.borrow().next {
                        if let Some(ref _prev) = node_iter.borrow().prev {
                            _prev.borrow_mut().next = Some(Rc::clone(_next));
                            tmp = Some(Rc::clone(_prev));
                        }
                        _next.borrow_mut().prev = tmp;
                    }
                    
                    find = true;
                    self.count -= 1;
                    break;
                }

                let node = Rc::clone(&node_iter);
                if node.borrow().next.is_some() {
                    node_iter = Rc::clone(node.borrow().next.as_ref().unwrap());
                }
            };
            find
        }
    }

    pub fn push_back(&mut self, val: i64) {
        let new_node = Rc::new(RefCell::new(ListNode::new(val)));
        if let Some(ref _next) = self.tail {
            if let Some(ref _prev) = _next.borrow_mut().prev {
                _prev.borrow_mut().next = Some(Rc::clone(&new_node));
                new_node.borrow_mut().prev = Some(Rc::clone(_prev));
            }
            new_node.borrow_mut().next = Some(Rc::clone(_next));
            _next.borrow_mut().prev = Some(Rc::clone(&new_node));
        }
        self.count += 1;
    }

    pub fn pop_back(&mut self) -> i64 {
        if self.count == 0 {
            println!("No nodes");
            -1
        } else {
            let mut val : i64 = -1;
            let mut tmp = None;
            if let Some(ref _tail) = self.tail {
                if let Some(ref _target) = _tail.borrow().prev {
                    val = _target.borrow().val;
                    if let Some(ref _prev) = _target.borrow().prev {
                        _prev.borrow_mut().next = Some(Rc::clone(_tail));
                        tmp = Some(Rc::clone(_prev));
                    }
                    // _target.borrow_mut().next = Some(None);
                    // _target.borrow_mut().prev = Some(None);
                }
                _tail.borrow_mut().prev = tmp;
            }
            self.count -= 1;
            val
        }
    }

    pub fn push_front(&mut self, val : i64) {
        let new_node = Rc::new(RefCell::new(ListNode::new(val)));
        if let Some(ref _head) = self.head {
            if let Some(ref _next) = _head.borrow().next {
                _next.borrow_mut().prev = Some(Rc::clone(&new_node));
                new_node.borrow_mut().next = Some(Rc::clone(&_next));
            }
            _head.borrow_mut().next = Some(Rc::clone(&new_node));
            new_node.borrow_mut().prev = Some(Rc::clone(&_head));
        }
        self.count += 1;
    }

    pub fn pop_front(&mut self) -> i64 {
        if self.count == 1 {
            println!("No nodes");
            -1
        } else {
            let mut tmp = None;
            let mut val : i64 = -1;
            if let Some(ref _head) = self.head {
                if let Some(ref _target) = _head.borrow().next {
                    if let Some(ref _next) = _target.borrow().next {
                        _next.borrow_mut().prev = Some(Rc::clone(&_head));
                        tmp = Some(Rc::clone(&_next));
                    }
                    val = _target.borrow().val;
                }
                _head.borrow_mut().next = tmp;
                // println!("drop here2");
            }
            self.count -= 1;
            val
        }
    }
}

impl Drop for ListNode {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with val = {}", self.val);
    }
}

// #[cfg(test)]

// mod test {
//     use super::*;

//     #[test]
    

//     #[test]
// }

fn test_new(){
    let _list = List::new();
}

fn test_push_back() {
    let mut list = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.walk_list();
    
    list.pop_back();
    list.walk_list();
    
    list.push_front(4);
    list.push_front(5);
    list.push_front(6);
    list.walk_list();

    list.pop_front();
    list.walk_list();

    list.insert_after(3, 9);
    list.walk_list();
    println!("index {} = {}", 2, list.at(2));

    list.take_out(9);
    list.walk_list();
}

fn main() {
    let _list = List::new();
    test_push_back();
    println!("after test push_back");
}