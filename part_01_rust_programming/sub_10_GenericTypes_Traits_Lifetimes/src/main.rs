/*
 * Every programming language has tools for effectively handling the duplication of concepts.
 * In Rust, one such tool is generics: abstract stand-ins for concrete types or other properties.
 */

 // ----------------------------------------------------------------------------- //
 // ---------------- demo 1: find the largest number from a list ---------------- //
 // ----------------------------------------------------------------------------- //
 // This codes find the largest number in a list

 #[allow(dead_code)]
 fn demo_find_largest_1list() {
     let number_list = vec![34, 50, 25, 100, 65];

     let mut largest = &number_list[0];

     for number in &number_list {
         if largest < number {
             largest = number
         }
     }

     println!("The largest number of number_list = {largest}")
 }

 // ------------------------------------------------------------------------------ //
 // ---------------- demo 2: find the largest number from 2 lists ---------------- //
 // ------------------------------------------------------------------------------ //
 // This codes find the largest number in 2 lists
 // => We duplicate that code 2 times

 #[allow(dead_code)]
 fn demo_find_largest_2lists() {
     let number_list_1 = vec![34, 50, 25, 100, 65];

     let mut largest = &number_list_1[0];

     for number in &number_list_1 {
         if largest < number {
             largest = number
         }
     }

     println!("The largest number of number_list_1 = {largest}");

     ///////////////////////////////////////////////////////

     let number_list_2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

     let mut largest = &number_list_2[0];

     for number in &number_list_2 {
         if number > largest {
             largest = number;
         }
     }

     println!("The largest number of number_list_2 = {largest}");
 }

 // --------------------------------------------------------------------------------------------------- //
 // ---------------- demo 3: extract the finding largest part to a standalone function ---------------- //
 // --------------------------------------------------------------------------------------------------- //

 #[allow(dead_code)]
 fn find_largest(list: &[i32]) -> &i32 {
     let mut largest = &list[0];

     for number in list {
         if number > largest {
             largest = number;
         }
     }

     largest
 }

 ////////////////////

  #[allow(dead_code)]
 fn demo_find_largest_function() {
     let number_list_1: Vec<i32> = vec![25, 12, 11, 14, 22, 28];
     let number_list_2: Vec<i32> = vec![47, 28, 38, 42, 44, 16];
     let number_list_3: Vec<i32> = vec![82, 90, 76, 92, 81, 87];

     let largest = find_largest(&number_list_1); // Use the standalone function, reduce a lot of duplication
     println!("The largest number of number_list_1 = {largest}");

     let largest = find_largest(&number_list_2);
     println!("The largest number of number_list_2 = {largest}");

     let largest = find_largest(&number_list_3);
     println!("The largest number of number_list_3 = {largest}");
 }

 // ################# //
 //       main()      //
 // ################# //

 fn main() {
     println!();

     // demo_find_largest_1list();
     // demo_find_largest_2lists();
     demo_find_largest_function()
 }
