// Vectors - Resizable arrays

use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![2, 3, 4, 3];

  // Re-assign value
  numbers[2] = 20;

  // Add on to vectors
  numbers.push(5);
  numbers.push(6);

  // Remove last value
  numbers.pop();

  println!("{:?}", numbers);
  // get single val
  println!("Single value: {}", numbers[0]);
  // get array length
  println!("Vector length: {}", numbers.len());
  // Vectors are stack allocated
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers));
  // get slice
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);
  // loop through vector values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }
  // loop and mutate values
  for x in numbers.iter_mut() {
    *x *= 2;
  }
  println!("Numbers Vec: {:?}", numbers);
}
