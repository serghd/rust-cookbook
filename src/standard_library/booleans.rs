#![allow(dead_code)]

pub fn evaluate_booleans() {
    let my_vec = vec![true, false, true, false, false];

    let opt_vec = my_vec
        .iter()
        .map(|item| {
            item.then(|| {
                println!("Got a {}", item);
                "It's true, you know"
            })
        })
        .collect::<Vec<_>>();

    println!("Opt vector: {:?}", opt_vec);

    let filtered_vec = opt_vec.into_iter().filter_map(|n| n).collect::<Vec<_>>();
    println!("Filtered vector: {:?}", filtered_vec);
}
