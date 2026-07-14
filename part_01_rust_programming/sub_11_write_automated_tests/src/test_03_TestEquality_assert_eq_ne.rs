/*
 * assert_eq!(input, test_value)
 * + if input == test_value => test passed, nothing happens
 * + else, panics
 *
 * assert_ne!(input, test_value)
 * + if input != test_value => test passed, nothing happens
 * + else, panics
 */

 pub fn add_two<T>(input: &T) -> T
 where
     for <'a> &'a T: std::ops::Add<i32, Output = T> { // T can be added to an i32
     input + 2
 }

 #[cfg(test)]
 mod tests {
     use super::*;

     #[test]
     fn test_equality() {
         let input = 35;
         let result = add_two(&input);
         assert_eq!(result, input + 2); // 37 == 35 + 2
     }

     #[test]
     fn test_inequality() {
         let input = 35;
         let result = add_two(&input);
         assert_ne!(result, input + 5); // 37 != 35 + 5
     }
 }
