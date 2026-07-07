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

 //////////////////////////////////
 // remind of Dangling reference //
 //////////////////////////////////

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

  ////////////////////
  // A good example //
  ////////////////////

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
   // --------------------------------------- Generic Lifetime in Functions --------------------------------------- //
   // ------------------------------------------------------------------------------------------------------------- //

   //////////////////////////////////////////////////////////////////
   // The problem: the compiler cannot guess what will be returned //
   //////////////////////////////////////////////////////////////////

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

   /////////////////////////////////////////////////////////////////////////////////////////////////////////////////
   // The solution - lifetime generic: hey compiler, you don't need to guess, just need one of them to live is ok //
   /////////////////////////////////////////////////////////////////////////////////////////////////////////////////

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
    *
    * In CONCLUSION, the function doesn’t need to know exactly how long x and y will live,
    * only that some scope can be substituted for 'a that will satisfy this signature.
    */

    ///////////////////////////////////////////
    // The Golden Rule: smaller takes it all //
    ///////////////////////////////////////////

    fn demo_lifetime_golden_rule() {
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
        /*
         * This line could not run. Why? Remember the ``Golden Rule of 'a``,
         * that is ``'a becomes the smaller lifetime.``
         *
         * In this case, ``string2`` lives in the inner scope, which has smaller lifetime,
         * so 'a is restricted to the inner scope.
         *
         * When the inner scope ends, ``string2`` dies, which means 'a also dies.
         * Therefore, ``result`` (which is tied to 'a) will become invalid outside this scope
         * => code panics
         */
    }

    // ----------------------------------------------------------------------------------------------------------- //
    // --------------------------------------- Lifetime annotations syntax --------------------------------------- //
    // ----------------------------------------------------------------------------------------------------------- //
    /*
     * Lifetime annotations don’t change how long any of the references live.
     * Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
     *
     * (Just as functions can accept any type when the signature specifies a generic type parameter,
     *  functions can accept references with any lifetime by specifying a generic lifetime parameter.)
     *
     * &i32        // a reference
     * &'a i32     // a reference with an explicit lifetime 'a
     * &'a mut i32 // a mutable reference with an explicit lifetime 'a
     *
     * ('a = apostrophe a)
     *
     * Here, we can use 'a or 'b or 'c, ...
     * but most people use the name 'a for the first lifetime annotation.
     */

     // ----------------------------------------------------------------------------------------------------------- //
     // --------------------------------------- Generic Lifetime in Structs --------------------------------------- //
     // ----------------------------------------------------------------------------------------------------------- //
     /*
      * Just like functions, structs can also hold references.
      *
      * But if a struct holds a reference,
      * the struct cannot outlive the data it is referencing.
      */

      struct ImportantExcerpt<'a> { // declare struct's name with a generic lifetime parameter 'a
          part: &'a str,
      }

      fn demo_struct_lifetime() {
          let novel = String::from("Call me Ishmael. Some years ago...");
          let first_sentence = novel.split('.').next().unwrap();
          let i = ImportantExcerpt {
              part: first_sentence,
          };
          println!("i.part = {}", i.part)
      }

      /*
       * Here, ImportantExcerpt requires a lifetime <'a>
       *
       * This tells the compiler:
       * "An instance of ImportantExcerpt cannot live longer than the string slice it holds in ``part``"
       *
       * Because ``novel`` (the owner of the data) lives until the end of the demo function,
       * the struct ``i`` is perfectly safe.
       */

       // --------------------------------------------------------------------------------------------------------- //
       // --------------------------------------- Lifetime Elision: 3 rules --------------------------------------- //
       // --------------------------------------------------------------------------------------------------------- //

       //////////////////
       // Introduction //
       //////////////////
       /*
        * As discussed, every reference has a lifetime
        * and that we need to specify lifetime parameters for functions or structs that use references.
        *
        * However, an old example like below without generic lifetime parameter still works
        */

        #[allow(dead_code)]
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }

        // In the old versions (Rust pre 1.0), this would not run unless the generic lifetime parameter is given like this

        #[allow(dead_code)]
        fn first_word_lifetime<'a>(s: &'a str) -> &'a str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }

        /*
         * After time, the Rust maintainance team found that
         * Rust programmers were entering the same lifetime annotations over and over in particular situations.
         *
         * These situations were predictable and followed a few deterministic patterns.
         * => the devs decided to program these patterns into the compiler
         * => the compiler now can automatically infer the lifetimes in these situations (without needing to specify)
         *
         * These patterns are called the ``lifetime elision rules``
         *
         * The elision rules don’t provide full inference.
         * If there is still ambiguity about what lifetimes the references have after Rust applies the rules,
         * you have to add lifetime annotations, otherwise the compiler will raise error
         */

         /////////////////////////////////////////////////////////////////////
         // Rule 1: Assign a lifetime parameter to each reference parameter //
         /////////////////////////////////////////////////////////////////////
         /*
          * For example:
          *
          * fn foo(x: &i32) -> &i32 {}
          * => fn foo<'a>(x: &'a i32) -> &i32 {}
          *
          * fn baz(x: &i32, y: &f64, z: u8) -> (&i64, &str) {}
          * => fn baz<'a, 'b>(x: &'a i32, y: &'b f64, z: u8) -> (&i64, &str) {}
          *
          * Here, ``z: u8`` is not assigned to any lifetime parameter
          * because it is not a reference parameter
          */

          //////////////////////////////////////////////////////////////////////////////////////////////////
          // Rule 2: If there is exactly one input lifetime parameter, assign it to all output lifetimes. //
          //////////////////////////////////////////////////////////////////////////////////////////////////
          /*
           * ////////////// Valid example //////////////
           *
           * fn foo<'a>(x: &'a i32) -> &i32 {}
           * => fn foo<'a>(x: &'a i32) -> &'a i32 {}
           *
           * fn foo<'a>(x: &'a i32, y: u8) -> (&i32, &str) {}
           * => fn foo<'a>(x: &'a i32, y: u8) -> (&'a i32, &'a str) {}
           *
           * As you can see in the second example,
           * the lifetime 'a of the single reference input x is applied for both outputs ``(&'a i32, &'a str)``
           * (``y: u8`` is not a reference parameter so it does not violate the rule)
           *
           * ////////////// Violation example //////////////
           *
           * fn baz<'a, 'b>(x: &'a i32, y: &'b f64, z: u8) -> (&i64, &str) {}
           *
           * In this case, both x and y are reference parameters with different lifetimes ('a and 'b),
           * so, the compiler could not determine to apply which one for the outputs.
           *
           * The only solution is that you have to specify the lifetime parameter yourself (like the ``longest()`` function above)
           */

           //////////////////////////////////////////////////////////////////////////////////////////
           // Rule 3:  If there are multiple input lifetimes, but one of them is &self (a method), //
           //          the lifetime of self is assigned to all output lifetimes.                   //
           //////////////////////////////////////////////////////////////////////////////////////////
           // See the next section for demonstration

           // ----------------------------------------------------------------------------------------------------------- //
           // ---------------------------- Generic Lifetime in Methods: the 3rd elision rule ---------------------------- //
           // ----------------------------------------------------------------------------------------------------------- //

           struct ImportantComponent<'a> {
               part: &'a str,
           }

           impl<'a> ImportantComponent<'a> {
               // Let's pretend we are the compiler applying the rules to this method:
               // fn announce_and_return_part(&self, announcement: &str) -> &str

               // STEP 1 (Rule 1): Assign a lifetime to each reference input.
               // Let's call &self's lifetime 'b, and announcement's lifetime 'c.
               // fn announce_and_return_part<'b, 'c>(&'b self, announcement: &'c str) -> &str

               // STEP 2 (Rule 2): Does Rule 2 apply? No, because there are TWO input lifetimes ('b and 'c).

               // STEP 3 (Rule 3): Are there multiple input lifetimes, but one is &self? YES!
               // Therefore, the output gets the lifetime of &self ('b).
               // fn announce_and_return_part<'b, 'c>(&'b self, announcement: &'c str) -> &'b str

               // Because the compiler does this automatically, we can just write it cleanly like this:
               fn announce_and_return_part(&self, announcement: &str) -> &str {
                   println!("Attention please: {}", announcement);
                   self.part // Returning a reference tied to &self's lifetime
               }
           }

           fn demo_method_lifetime() {
               let novel = String::from("Call me Ishmael. Some years ago...");
               let first_sentence = novel.split('.').next().unwrap();
               let i = ImportantComponent {
                   part: first_sentence,
               };

               let i_part = i.announce_and_return_part("lady and gentlemen!");

               println!("i.part = {}", i_part)
           }

           // ----------------------------------------------------------------------------------------------- //
           // --------------------------------------- Static Lifetime --------------------------------------- //
           // ----------------------------------------------------------------------------------------------- //
           /*
            * `` 'static `` is a special lifetime,
            * it denotes that the affected reference can live for the ENTIRE DURATION of the program.
            *
            * For example: all string literals have the 'static lifetime
            */

            fn demo_static_lifetime() {
                let s: &'static str = "I have a static lifetime.";
                println!("s = {s}");
            }

            /*
             * The text of ``s`` is stored directly in the program’s binary, which is always available.
             * Therefore, the lifetime of all string literals is 'static.
             *
             * Before specifying 'static as the lifetime for a reference,
             * think about whether or not the reference you have actually lives the entire lifetime of your program,
             * and whether you want it to.
             *
             * Most of the time, an error message suggesting the 'static lifetime results from attempting to create a dangling reference
             * or a mismatch of the available lifetimes.
             * In such cases, the solution is to fix those problems, not to specify the 'static lifetime.
             */

 // ################# //
 //       main()      //
 // ################# //

 fn main() {
     println!();

     demo_good_lifetime();

     println!("\n===================================================================\n");

     // demo_longest();
     demo_longest_generic_lifetime();
     demo_lifetime_golden_rule();

     println!("\n===================================================================\n");

     demo_struct_lifetime();

     println!("\n===================================================================\n");

     demo_method_lifetime();

     println!("\n===================================================================\n");

     demo_static_lifetime();
 }
