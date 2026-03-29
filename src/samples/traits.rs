use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
struct Animal {
    name: String,
}

trait Dog {
    fn bark(&self);
    fn run(&self);
}

impl Dog for Animal {
    fn bark(&self) {
        println!("{} is barking!", self.name);
    }
    fn run(&self) {
        println!("{} is running!", self.name);
        println!("Animal object: {:?}", self);
    }
}

impl fmt::Display for Animal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Animal name is: {}", self.name)
    }
}

pub fn evaluate_trait() {
    let dog = Animal { name: "Dog".to_string() };
    dog.bark();
    dog.run();

    println!("{}", dog);
}

////////////////////////////////////////////

#[derive(Debug)]
struct Monster {
    health: i32,
}

#[derive(Debug)]
struct Wizard {
    _health: i32,
}

#[derive(Debug)]
struct Knight {
    _health: i32,
}

trait FightClose {}
trait FightFromDistance {}

impl FightFromDistance for Wizard {}
impl FightClose for Knight {}

fn attack_with_fireball<T: FightFromDistance + Debug>(character: &T, opponent: &mut Monster) {
    opponent.health -= 10;
    println!(
        "You attack with your bow. Your opponent now has {} health left.  You are now at: {:?}",
        opponent.health, character
    );
}

fn attack_with_sword<T: FightClose + Debug>(character: &T, opponent: &mut Monster) {
    opponent.health -= 10;
    println!(
        "You attack with your sword. Your opponent now has {} health left. You are now at: {:?}",
        opponent.health, character
    );
}

pub fn evaluate_trait_bounds() {
    let wizard = Wizard { _health: 100 };
    let knight = Knight { _health: 200 };
    let mut monster = Monster { health: 150 };
    attack_with_fireball(&wizard, &mut monster);
    attack_with_sword(&knight, &mut monster);
}

////////////////////////////////////////////

#[derive(Debug)]
struct City {
    name: String,
    population: u32,
}

impl City {
    pub fn create(name: &str, population: u32) -> City {
        City { name: name.to_string(), population }
    }
}

struct Country {
    cities: Vec<City>,
}

impl From<Vec<City>> for Country {
    fn from(cities: Vec<City>) -> Self {
        Self { cities }
    }
}

impl Country {
    pub fn print_cities(&self) {
        for city in &self.cities {
            println!("City {:?} has population {:?}", city.name, city.population);
        }
    }
}

pub fn evaluate_trait_from() {
    let city1 = City::create("City 1", 100);
    let city2 = City::create("City 2", 987);

    let cities = vec![city1, city2];
    let country = Country::from(cities);

    country.print_cities();
}

////////////////////////////////////////////

pub fn print_string_as_bytes<T>(input: T)
where
    T: AsRef<[u8]> + Debug,
{
    println!("{:?}", input.as_ref());
}

struct User {
    name: String,
    _age: u32,
}

impl AsRef<str> for User {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

fn print_as_ref<T>(input: T)
where
    T: AsRef<str>,
{
    println!("{}", input.as_ref());
}

pub fn print_objects_as_ref() {
    print_as_ref("abc");
    print_as_ref(String::from("abc (String)"));
    print_as_ref(User { name: "Nick".to_string(), _age: 21 })
}

pub fn traits_print_string_as_bytes() {
    print_string_as_bytes("abc");
}

////////////////////////////////////////////

pub fn gives_higher_i32<T: PartialOrd + Display>(num1: T, num2: T) {
    let higher = if num1 > num2 { num1 } else { num2 };
    println!("higher: {}", higher);
}

pub fn traits_gives_higher_i32() {
    gives_higher_i32(8, 10);
}

////////////////////////////////////////////

fn prints_with_impl_trait(input: impl Into<String> + Display) {
    println!("Your input: {}", input);
}

pub fn print_with_impl_trait() {
    let str1 = "abc";
    let str2 = "Some words".to_string();
    prints_with_impl_trait(str1);
    prints_with_impl_trait(str2);
}

////////////////////////////////////////////

fn returns_a_closure(input: &str) -> impl FnMut(i32) -> i32 {
    match input {
        "double" => |mut number| {
            number *= 2;
            println!("Doubled value: {}", number);
            number
        },
        "triple" => |mut number| {
            number *= 3;
            println!("Tripled value: {}", number);
            number
        },
        _ => |number| {
            println!("Unknown operation");
            number
        },
    }
}

pub fn call_returned_closures() {
    let mut doubled_fun = returns_a_closure("double");
    let mut tripled_fun = returns_a_closure("triple");
    let mut other_fun = returns_a_closure("other");
    doubled_fun(10);
    tripled_fun(5);
    other_fun(10);
}
