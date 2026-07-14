/*
 * The ``assert!`` macro, provided by the standard library,
 * is useful when you want to ensure that some condition in a test evaluates to ``true``.
 *
 * If the value is ``true``, nothing happens and the test passes.
 * If the value is ``false``, the assert! macro calls panic! to cause the test to fail
 *
 * assert!(condition)
 *
 * ##########################
 *
 * cargo test -p sub_11_write_automated_tests -- test_02_CheckResult_assert_boolean
 */

 #[derive(Debug)]
 struct Rectangle<T: std::cmp::PartialOrd> { // Use PartialOrd
     width: T,
     height: T,
 }

 impl<T: std::cmp::PartialOrd> Rectangle<T> {
     fn can_hold(&self, other: &Rectangle<T>) -> bool {
         (self.width > other.width) && (self.height > other.height)
     }
 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {width: 26.5, height: 11.79};
        let smaller = Rectangle {width: 15.2, height: 7.3};

        let result = larger.can_hold(&smaller);
        assert!(result);
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {width: 26.5, height: 11.79};
        let smaller = Rectangle {width: 15.2, height: 7.3};

        assert!(!smaller.can_hold(&larger));
        // the ``smaller.can_hold(&larger)`` will return ``false`` -> makes the ``assert!`` fail
        // therefore, must use ``!smaller.can_hold(&larger)`` to negate the false into ``true`` -> now ``assert!`` succeeds
    }

}
