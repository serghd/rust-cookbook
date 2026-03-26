#![allow(dead_code)]

pub fn print_cities() {
    let cities = ["Beirut", "Tel Aviv", "Nicosia"];
    let [city1, _city2, _city3] = cities;
    println!("{:?}", city1);
}
