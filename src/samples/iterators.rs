pub fn evaluate_iterators_1() {
   let vec = vec![1, 2, 3];
   let vec_a = vec.iter().map(|x| x + 1).collect::<Vec<i32>>();
   let vec_b = vec.into_iter().map(|x| x + 10).collect::<Vec<i32>>();
   let mut vec_c = vec![1, 2, 3];
   vec_c.iter_mut().for_each(|x| *x *= 10);

   println!("{:?}", vec_a);
   println!("{:?}", vec_b);
   println!("{:?}", vec_c);
}

/////////////////////////////////////////

#[derive(Clone)]
struct Library {
   lib_type: LibraryType,
   books: Vec<String>,
}

#[derive(Clone)]
enum LibraryType {
   city,
   country,
}

impl Library {
   pub fn new() -> Self {
      Self { lib_type: LibraryType::city, books: Vec::new() }
   }

   pub fn add_book(&mut self, title: &str) {
      self.books.push(title.to_string());
   }
}

impl Iterator for Library {
   type Item = String;

   fn next(&mut self) -> Option<Self::Item> {
      match self.books.pop() {
         Some(book) => Some(book + " found!"),
         None => None,
      }
   }
}

pub fn evaluate_iterators_2() {
   let mut library = Library::new();
   library.add_book("my book 1");
   library.add_book("my book 2");
   library.add_book("my book 3");

   for item in library.clone() {
      println!("{:?}", item);
   }
}
