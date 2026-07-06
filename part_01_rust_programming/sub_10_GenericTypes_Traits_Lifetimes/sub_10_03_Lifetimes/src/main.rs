/*
 * Lifetimes are another kind of generic that we’ve already been using.
 * Lifetimes ensure that references are valid as long as we need them to be.
 *
 * As discussed before (ownership and borrowing),
 * every reference in Rust has a lifetime, which is the scope for which that reference is valid.
 *
 * Most of the time, lifetimes are implicit and inferred.
 * Just like most of the time, types are inferred. We are only required to annotate types when multiple types are possible.
 * In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways.
 *
 * Rust requires us to annotate the relationships using generic lifetime parameters
 * to ensure that the actual references used at runtime will definitely be valid.
 *
 */


 // -------------------------------------------------------------------------------------------------------------- //
 // --------------------------------------- The dangling reference problem --------------------------------------- //
 // -------------------------------------------------------------------------------------------------------------- //

 // fn demo_dangling_reference() {
 //     let r;                // ---------+-- 'a
 //                           //          |
 //     {                     //          |
 //         let x = 5;        // -+-- 'b  |
 //         r = &x;           //  |       |
 //     }                     // -+       |
 //                           //          |
 //     println!("r: {r}");   //          |
 // }                         // ---------+

 /*
  * In this code:
  * + 'a represents the lifetime of r (outer scope)
  * + 'b represents the lifetime of x (inner scope)
  *
  * At compile time, the Rust compiler will compare the two lifetimes,
  * then it sees that ``r`` has a lifetime of 'a,
  * and it's trying to refer to a memory ``x`` with a lifetime of 'b,
  *
  * However, as you can see in the diagram, 'b is shorter than 'a.
  * So, when we try to use ``r`` in ``println!("r: {r}")``,
  * ``r`` has nothing for the program to use -> panic
  *
  * Therefore, this program will be rejected because:
  * The subject of the reference (x) doesn’t live as long as the reference (r).
  *
  * => This is called a dangling reference
  */

  ////////////////////////////////////////////////////////////

  fn demo_good_lifetime() {
      let x = 5;            // ----------+-- 'b
                            //           |
      let r = &x;           // --+-- 'a  |
                            //   |       |
      println!("r: {r}");   //   |       |
                            // --+       |
  }                         // ----------+

  /*
   * This case is opposite, 'b is longer than 'a,
   * meaning the subject of reference (x) lives longer than the reference (r),
   * so ```println!("r: {r}");``` now has something to use, no more dangling reference
   */

   // ------------------------------------------------------------------------------------------------------------- //
   // --------------------------------------- Generic Lifetime in functions --------------------------------------- //
   // ------------------------------------------------------------------------------------------------------------- //

   //////////////////////// The Problem ////////////////////////

   // fn demo_longest() {
   //     fn longest(x: &str, y: &str) -> &str {
   //         if x.len() > y.len() { x } else { y }
   //     }

   //     let string1 = String::from("abcd");
   //     let string2 = "xyz";

   //     let result = longest(string1.as_str(), string2);
   //     println!("The longest string is {result}");
   // }

   /*
    * If you try to compile this, Rust throws an error: ```missing lifetime specifier```.
    *
    * Why? Because Rust can’t tell whether the reference being returned refers to x or y
    * The ``if`` block returns ``x``,
    * but the ``else`` block returns ``y``
    *
    * When we’re defining this function, we don’t know the concrete values that will be passed into this function,
    * so we don’t know whether the ``if`` case or the ``else`` case will execute.
    *
    * The borrow checker can’t determine this either,
    * because it doesn’t know how the lifetimes of ``x`` and ``y`` relate to the lifetime of the return value.
    *
    * It refuses to guess, because guessing wrong could lead to a dangling reference.
    */

   //////////////////////// The SOLUTION ////////////////////////

   fn demo_longest_generic_lifetime() {
       fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
           if x.len() > y.len() { x } else { y }
       }

       let string1 = String::from("abcd");
       let string2 = "xyz";

       let result = longest(string1.as_str(), string2);
       println!("The longest string is {result}");
   }

   /*
    * In this code,
    * we declare <'a> after the function name to say "This function involves a lifetime we will call 'a".
    * then, we tell Rust:
    * + ``x: &'a str``: "Take parameter x (which lives for at least 'a),
    * + ``y: &'a str``: and parameter y (which also lives for at least 'a)."
    *
    * We promise ``-> &'a str``: "The string slice I return will also live for at least 'a."
    *
    * The Golden Rule of 'a: In practice, 'a will become the smaller of the two lifetimes of x and y.
    * The returned reference is only guaranteed to be valid for the overlapping scope where both x and y are valid.
    */

    //////////////////////////////////////////////////

    fn demo_longest_generic_lifetime_innerscope() {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() { x } else { y }
        }

        let string1 = String::from("abcd");

        { // inner scope
            let string2 = "xyz";
            let result = longest(string1.as_str(), string2);
            println!("The longest string is {result}");
        }

        // println!("The longest string is {result}");
        // This line cannot run because the lifetime of ``result`` is no longer valid out of the inner scope
    }

 // ################# //
 //       main()      //
 // ################# //

 fn main() {
     println!();

     demo_good_lifetime();

     println!("\n===================================================================\n");

     // demo_longest();
     demo_longest_generic_lifetime();
     demo_longest_generic_lifetime_innerscope();

 }
