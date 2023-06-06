fn main() {
  // Declare a tuple of three elements
  let (letter, number, bool_value) = ('E', 5i32, true);

  // Use tuple indexing and show the values of the elements in the tuple
  println!("Is '{}' the {}th letter of the alphabet? {}", letter, number, bool_value);
}