/* first test */
// left in here as litter and for personal historical value

pub fn add(a: i32, b: i32) -> i32 {
  a + b
}

#[cfg(test)]
mod tests {
  use super::*; // import functions

  #[test]
  fn test_add() {
    assert_eq!(add(1, 2), 3);
  }
}