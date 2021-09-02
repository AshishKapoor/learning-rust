pub fn run() {
  // primitive string = immutable fixed-length string somewhere in the memory
  let mut hello = String::from("hello "); // String::from brings in string methods
                                          // Get length
  println!("Length {}", hello.len());
  // push char
  hello.push('W');
  // push string
  hello.push_str("orld!");
  // capacity in bytes
  println!("Capacity: {}", hello.capacity());
  // is empty check
  println!("Is Empty: {}", hello.is_empty());
  // contains
  println!("Contains 'World': {}", hello.contains("World"));
  // replace
  println!("replace: {}", hello.replace("World", "There"));

  // loop through string by whitespace
  for word in hello.split_whitespace() {
    println!("{}", word);
  }
  // Create string with capacity
  let mut str = String::with_capacity(10);
  str.push('a');
  str.push('b');
  println!("{}", str);
  // Assertion testing
  assert_eq!(2, str.len());
  assert_eq!(10, str.capacity());
  println!("print: {}", hello);
}
