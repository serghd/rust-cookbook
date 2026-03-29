pub struct City {
    name: String,
    name_before: String,
    population: u32,
    date_founded: u32,
}

impl City {
    pub fn create(name: String, name_before: String, population: u32, date_founded: u32) -> Self {
        City { name, name_before, population, date_founded }
    }

    pub fn process_city(&self) {
        let City { name, name_before, population: _population, date_founded: _date_founded } = self;
        let two_names = vec![name, name_before];
        println!("City's two names are: {:?}", two_names)
    }
}

pub fn destruct_city() {
    let city = City::create("City New".to_string(), "City Old".to_string(), 5000, 1219);
    city.process_city();
}
