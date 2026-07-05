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
 // --------------------- Define a Trait and implment it on different data types -------------------------- //
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

  pub struct NewsArticle { // define NewsArticle data type
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

  pub struct SocialPost { // define SocialPost data type
      pub username: String,
      pub content: String,
      pub reply: bool,
      pub repost: bool
  }

  impl Summary for SocialPost { // implement the Summary trait on SocialPost
      fn summarize(&self) -> String { // define the custom behaviour of ``summarize()`` method for NewsArticle
          format!("{}: {}", self.username, self.content)
      }
  }

  ////////////////////

  // use aggregator::{SocialPost, Summary}; // do this if you write the above codes in a separate crate named ``aggregator.rs``

  fn demo_trait() {
      let post = SocialPost {
          username: String::from("horse_ebooks"),
          content: String::from(
              "of course, as you probably already know, people",
          ),
          reply: false,
          repost: false,
      };

      let article = NewsArticle {
          headline: String::from("Brand new day!"),
          location: String::from("New York"),
          author: String::from("Mysterious_fat_man"),
          content: String::from("This day is the best day of my life!")
      };

      println!("post.summarize() = {}", post.summarize()); // implement custom behaviour for type SocialPost
      println!("article.summarize() = {}", article.summarize()) // implement custom behaviour for type NewsArticle
  }

  // ------------------------------------------------------------------------------------------------------- //
  // -------------------------- One trait with many methods - default method ------------------------------- //
  // ------------------------------------------------------------------------------------------------------- //
  /*
   * Sometimes it’s useful to have default behavior for some or all of the methods in a trait
   * instead of requiring implementations for all methods on every type.
   */

   pub trait GetInfor {
       fn get_author(&self) -> String; // this trait is type specific, differs across types

       fn get_summary(&self) -> String { // This trait is default, all types have the same behaviour like this
           format!("(Read more from {}...)", self.get_author())
       }
   }

   impl GetInfor for NewsArticle {
       fn get_author(&self) -> String { // define behaviour of ``get_author()`` method for NewsArticle
           format!("@{}", self.author)
       }
   }

   impl GetInfor for SocialPost {
       fn get_author(&self) -> String { // define behaviour of ``get_author()`` method for SocialPost
           format!("@{}", self.username)
       }
   }

   //////////////

   fn demo_trait_default() {
       let post = SocialPost {
           username: String::from("horse_ebooks"),
           content: String::from(
               "of course, as you probably already know, people",
           ),
           reply: false,
           repost: false,
       };

       let article = NewsArticle {
           headline: String::from("Brand new day!"),
           location: String::from("New York"),
           author: String::from("Mysterious fat man"),
           content: String::from("This day is the best day of my life!")
       };

       println!("post.get_summary() = {}", post.get_summary());
       println!("article.get_summary() = {}", article.get_summary())

       /*
        * Though we did not define custom behaviour of ``get_summary()`` for each type,
        * the codes still work be cause we already define a default behaviour for ``get_summary()``
        */
   }

   // ------------------------------------------------------------------------------------------------------------------- //
   // -------------------------- Use Trait as parameters in a function - ``where`` clause ------------------------------- //
   // ------------------------------------------------------------------------------------------------------------------- //
   /*
    * Trait can be used as parameters of a function as well,
    * to better demonstrate this, let's use 3 traits:
    * + ``Switchable``: the device that can be turned on and off
    * + ``Dimmable``: the device's intensity/brightness can be adjusted
    * + ``Network``: the device can connect to wifi
    *
    * and 3 devices (types):
    * + ``SmartBulb``: can be switched and dimmed
    * + ``SmartThermostat``: can be switched and connected to Wi-Fi
    * + ``StandardToaster``: can only be switched. It has no Wi-Fi and no dimmer
    *
    * Refers to crate ``smart_devices.rs``
    */

    #[allow(dead_code)]
    mod smart_devices;
    use crate::smart_devices::{
        Dimmable, Network, SmartBulb, SmartThermostat, StandardToaster, Switchable
    };

    ////////////////////////////
    // Use trait as parameter //
    ////////////////////////////

    fn activate_device(device: &mut impl Switchable) {
        // the ``&impl Switchable`` tells Rust to accept any ``device`` that has Switchable trait
        // Here, we use ``&mut impl Switchable`` to make it mutable
        println!("Activating...");
        device.switch_on();
    }

    fn demo_trait_parameter() {
        let mut bulb = SmartBulb {switch: false, dim: true};
        let mut thermostat = SmartThermostat{switch: false, wifi: false};

        activate_device(&mut bulb);
        println!("bulb.switch = {}", bulb.switch);

        activate_device(&mut thermostat);
        println!("thermostat.switch = {}", thermostat.switch)
    }

    ////////////////////////////////////////////////////////
    // Trait bound syntaxe <T: Trait> - trait and generic //
    ////////////////////////////////////////////////////////
    // The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound

    fn deactivate_device<T: Switchable>(device: &mut T) {
        /*
         * <T: Switchable> means we use T as a type generic placeholder (for SmartBulb, SmartThermostat, StandardToaster)
         * but that generic must implement trait ``Switchable``
         */
        println!("Deactivating...");
        device.switch_off();
    }

    fn demo_trait_bound_generic() {
        let mut bulb = SmartBulb {switch: true, dim: true};
        let mut toaster = StandardToaster{switch: true};

        deactivate_device(&mut bulb);
        println!("bulb.switch = {}", bulb.switch);

        deactivate_device(&mut toaster);
        println!("toaster.switch = {}", toaster.switch)
    }

    /////////////////////////////////////////////////////////
    // Multiple trait bounds with ``<T: Trait1 + Trait2>`` //
    /////////////////////////////////////////////////////////
    // What if we want to implement multiple traits at once? -> use ``<T: Trait1 + Trait2>``

    fn setup<T: Switchable + Dimmable>(device: &mut T) { // or ``fn setup(device: &mut (impl Switchable + Dimmable))``
        println!("Setting up...");
        device.switch_on();
        device.brighten();
    }

    fn demo_trait_bound_multiple() {
        let mut bulb = SmartBulb{switch: false, dim: true};

        setup(&mut bulb);

        println!("bulb.switch = {}", bulb.switch);
        println!("bulb.dim = {}", bulb.dim);

        /*
         * let mut thermostat = SmartThermostat{switch: false, wifi: false};
         * setup(&mut thermostat);
         *
         * => This code will fail be cause ``thermostat`` does not have trait ``Dimmable`` for method ``brighten()`` to work
         */
    }

    ////////////////////////////////////////////////
    // Multiple trait bounds and ``where`` clause //
    ////////////////////////////////////////////////
    /*
     * The above example only accepts types that implement both Switchable and Dimmable.
     * But how about other types? How to make a more flexible ``setup()`` function that can accept more?
     * => Use ``where`` clause
     */

     fn setup_devices<B, T>(bulb: &mut B, thermostat: &mut T)
     where
         B: Switchable + Dimmable, // bulb bounds to generic B which implements Switchable + Dimmable
         T: Switchable + Network, // thermostat bounts to generic T which implements Switchable + Network
     {
         println!("Setting up devices...");

         bulb.switch_on();
         bulb.brighten();

         thermostat.switch_on();
         thermostat.connect();
     }

     fn demo_trait_bound_where() {
         let mut bulb = SmartBulb {switch: false, dim: true};
         let mut thermostat = SmartThermostat{switch: false, wifi: false};

         setup_devices(&mut bulb, &mut thermostat);

         println!("bulb.switch = {}", bulb.switch);
         println!("bulb.dim = {}", bulb.dim);

         println!("thermostat.switch = {}", thermostat.switch);
         println!("thermostat.wifi = {}", thermostat.wifi);
     }

     // --------------------------------------------------------------------------------------------------------- //
     // -------------------------- Use Trait in the return position of a function ------------------------------- //
     // --------------------------------------------------------------------------------------------------------- //

     fn returns_summarizable() -> impl Summary {
         SocialPost {
             username: String::from("horse_ebooks"),
             content: String::from(
                 "of course, as you probably already know, people",
             ),
             reply: false,
             repost: false,
         }
     }

     fn demo_trait_return_position() {
         let summarizable = returns_summarizable();
         println!("summarizable.summarize() = {}", summarizable.summarize())
     }

     /*
      * However, you can only use ``impl Trait`` at return position if you’re returning a single type,
      * for example, this code below WILL NOT WORK because it could return two different types (NewsArticle or SocialPost)
      */
      // fn returns_summarizable(switch: bool) -> impl Summary {
      //     if switch {
      //         NewsArticle {
      //             headline: String::from(
      //                 "Penguins win the Stanley Cup Championship!",
      //             ),
      //             location: String::from("Pittsburgh, PA, USA"),
      //             author: String::from("Iceburgh"),
      //             content: String::from(
      //                 "The Pittsburgh Penguins once again are the best \
      //                  hockey team in the NHL.",
      //             ),
      //         }
      //     } else {
      //         SocialPost {
      //             username: String::from("horse_ebooks"),
      //             content: String::from(
      //                 "of course, as you probably already know, people",
      //             ),
      //             reply: false,
      //             repost: false,
      //         }
      //     }
      // }

      // -------------------------------------------------------------------------------------------------- //
      // -------------------- Use Trait Bounds to Conditionally Implement Methods ------------------------- //
      // -------------------------------------------------------------------------------------------------- //
      /*
       * By using a trait bound with an impl block that uses generic type parameters,
       * we can implement methods conditionally for types that implement the specified traits.
       */

      use std::fmt::Display;

      struct Pair<T> {
          x: T,
          y: T,
      }

      impl<T> Pair<T> {
          fn new(x: T, y: T) -> Self {
              Self { x, y }
          }
      }

      impl<T: Display + PartialOrd> Pair<T> { // only implement this on types that have traits Display and PartialOrd
          fn cmp_display(&self) {
              if self.x >= self.y {
                  println!("The largest member is x = {}", self.x);
              } else {
                  println!("The largest member is y = {}", self.y);
              }
          }
      }

      fn demo_trait_conditional() {
          let pair = Pair::new(2.3, -1.2);
          pair.cmp_display();
      }

 // ################# //
 //       main()      //
 // ################# //

 fn main() {
     println!();

     demo_trait();

     println!("\n===================================================================\n");

     demo_trait_default();

     println!("\n===================================================================\n");

     demo_trait_parameter();
     println!();
     demo_trait_bound_generic();
     println!();
     demo_trait_bound_multiple();
     println!();
     demo_trait_bound_where();

     println!("\n===================================================================\n");

     demo_trait_return_position();

     println!("\n===================================================================\n");

     demo_trait_conditional();
 }
