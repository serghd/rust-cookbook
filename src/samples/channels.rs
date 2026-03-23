#![allow(dead_code)]

use std::sync::mpsc::channel;

pub fn use_channels() {
    let (sender, receiver) = channel();
    let huge_vec = vec![0; 1000_000];
    let mut handle_vec = vec![];
    let mut receiver_vec = vec![];

    for i in 0..10 {
        let sender_clone = sender.clone();
        let mut work_vec: Vec<u8> = Vec::with_capacity(huge_vec.len() / 10);
        work_vec.extend(&huge_vec[i * 100_000..(i + 1) * 100_000]);
        let handle = std::thread::spawn(move || {
            for number in work_vec.iter_mut() {
                *number = 1;
            }
            sender_clone.send(work_vec).unwrap();
        });
        handle_vec.push(handle);
    }

    for item in handle_vec {
        item.join().unwrap();
    }

    while let Ok(result) = receiver.try_recv() {
        receiver_vec.push(result);
    }

    let result_vec = receiver_vec.into_iter().flatten().collect::<Vec<u8>>();
    println!(
        "first 10 elements: {:?}, last 10 elements: {:?}, total: {}",
        &result_vec[0..10],
        &result_vec[result_vec.len() - 10..],
        &result_vec.len()
    );
}
