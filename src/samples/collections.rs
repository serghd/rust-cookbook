#![allow(dead_code)]

use std::collections::{BinaryHeap, HashMap, VecDeque};

pub fn make_survey() {
    let data = vec![("male", 1), ("female", 2), ("female", 4), ("male", 10), ("male", 1)];

    let mut rating = HashMap::new();
    for item_pair in data {
        rating.entry(item_pair.0).or_insert(Vec::new()).push(item_pair.1);
    }
    for item_pair in rating {
        println!("Gender {:?}, votes {:?}", item_pair.0, item_pair.1);
    }
}

fn show_remaining_bheap(input: &BinaryHeap<i32>) -> Vec<i32> {
    let mut result = vec![];
    for item in input {
        result.push(*item);
    }
    result
}

pub fn binary_heap_demo() {
    let data = vec![12, 200, 1, 8, 34, 67, 9];
    let mut b_heap = BinaryHeap::new();
    for item in data {
        b_heap.push(item);
    }
    while let Some(item) = b_heap.pop() {
        println!("Current item: {}, items left: {:?}", item, show_remaining_bheap(&b_heap))
    }
}

fn show_remaining_vdeque(my_deque: &VecDeque<(&str, bool)>) {
    for item_pair in my_deque {
        if item_pair.1 == false {
            println!("You must: {}", item_pair.0);
        }
    }
}

fn done_last_job(my_deque: &mut VecDeque<(&str, bool)>) {
    if let Some(mut obj) = my_deque.pop_back() {
        obj.1 = true;
        my_deque.push_front(obj);
    }
}

pub fn vec_deque_demo() {
    let jobs = vec!["do thing #1", "do thing #2", "do thing #3"];
    let mut my_deque = VecDeque::new();
    for job in jobs {
        my_deque.push_back((job, false));
    }

    done_last_job(&mut my_deque);
    done_last_job(&mut my_deque);

    show_remaining_vdeque(&my_deque);

    for item_pair in my_deque {
        println!("{:?}", item_pair)
    }
}
