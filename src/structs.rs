// structs - used to create custom data types

// Traditional struct
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

// Tuple struct
struct ColorTuple(u8, u8, u8);

struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string(),
    }
  }
  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }
  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}

pub fn run() {
  // Traditional struct
  let mut c = Color {
    red: 255,
    green: 0,
    blue: 0,
  };
  c.red = 200;
  println!("Color: {} {} {}", c.red, c.green, c.blue);

  // Tuple struct
  let mut c_t = ColorTuple(255, 0, 0);
  c_t.0 = 200;
  println!("Color: {} {} {}", c_t.0, c_t.1, c_t.1);

  // Person block
  let mut p = Person::new("Marry", "Doe");
  println!("Person: {} {}", p.first_name, p.last_name);
  println!("Full name: {}", p.full_name());
  p.set_last_name("Kapoor");
  println!("Full name: {}", p.full_name());
  println!("Person Tuple: {:?}", p.to_tuple());
}
