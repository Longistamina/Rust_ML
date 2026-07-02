/*
 * A ``trait`` defines the functionality a particular type has and can share with other types.
 *
 * We can use traits to define shared behavior in an abstract way,
 * and use trait bounds to specify the behaviours that a generic should have.
 * (like the trait ``std::cmp::PartialOrd`` mentioned in the file ``01_Generics.rs``)
 *
 * Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.
 * (in Python, it is polymorphism)
 *
 * A type can have different methods to perform different behaviours.
 * Some types can perform the same behaviour if we call a same method on them.
 * For example:
 * + ``2.4f64.abs()``
 * + ``(-3i8).abs()``
 * => Here, the method ``abs()`` performs the same kind of behaviour on both types f64 and i8
 */

 // ------------------------------------------------------------------------------------------------------- //
 // ------------- Define a Trait and implment it on different data types ---------------------------------- //
 // ------------------------------------------------------------------------------------------------------- //
 /*
  * Let’s say we have multiple structs that hold various kinds and amounts of text:
  * + ``NewsArticle``: a struct that holds a news story filed in a particular location
  * + ``SocialPost``: a struct that holds a kind of text that can have, at most, 280 characters
  *                   along with metadata that indicates whether it was a new post, a repost, or a reply to another post.
  *
  * Then, we also want to create a functionality to summarize the data stored in these 2 types,
  * we use it by calling ``instant.summarize()``.
  *
  * So, the ``summarize()`` method can work on both types
  *
  * For the same method, different types can have custom behaviour.
  */

  pub trait Summary { // Declare a trait using ``trait`` keyword, then the trait's name ``Summary``
                      // we also use ``pub`` so that other crates depending on this crate can use this trait too

        fn summarize(&self) -> String; // define a method signatures that describe the behaviour of the types implementing this trait
                                       // We stop here by ``;``, we will define the custom behaviour for each type later
  }

  ////////////////////

  pub struct NewsArticle {
      pub headline: String,
      pub location: String,
      pub author: String,
      pub content: String,
  }

  impl Summary for NewsArticle { // implement the Summary trait on NewsArticle
      fn summarize(&self) -> String { // define the custom behaviour of ``summarize()`` method for NewsArticle
          format!("{}, by {} ({})", self.headline, self.author, self.location)
      }
  }

  ////////////////////

 // ################# //
 //       main()      //
 // ################# //

 fn main() {
 }
