/*
 * We can also write tests that use ``Result<T, E>``
 *
 * Writing tests so that they return a Result<T, E> enables you to use the question mark operator "?" in the body of tests,
 * which can be a convenient way to write tests that should fail if any operation within them returns an Err variant.
 *
 * You can’t use the #[should_panic] annotation on tests that use ``Result<T, E>.``
 *
 * To assert that an operation returns an Err variant,
 * don’t use the question mark operator on the Result<T, E> value.
 * Instead, use assert!(value.is_err()).
 */

 pub fn add<T: std::ops::Add<Output = T>>(left: T, right: T) -> T { // The function to be tested
     left + right
 }

 #[cfg(test)]
 mod tests {
     use super::*;

     #[test]
     fn it_works() -> Result<(), String> { // return a ``Result<(), String>`` type
         let result = add(2, 2);

         if result == 4 {
             Ok(())
         } else {
             Err(String::from("two plus two does not equal four"))
         }
     }
 }
