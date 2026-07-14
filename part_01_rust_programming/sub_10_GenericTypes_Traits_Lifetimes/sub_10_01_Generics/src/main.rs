/*
 * In the file ``00_introduction_ReduceDuplicateCodes.rs``,
 * we define a ``find_largest(list: &[i32]) -> &i32`` funtion to avoid duplicating codes.
 * But this ``find_largest()`` only works on ``list [i32]``.
 *
 * What if we have another list of f64? Or a slice of char?
 * In this case, we will have to create new specific functions for them,
 * but this again leads to code duplications, for example:
 * + ``find_largest_float(list: &[f64]) -> &f64``
 * + ``find_largest_char(list: &[char]) -> &char``
 * => codes duplication
 *
 * To solve this, Rust offers ``GENERIC``.
 * Generics help us create definitions for items like function signatures or structs,
 * which we can then use with many different concrete data types (i32, f64, String, ...).
 *
 * Particularly, when defining a function that uses generics,
 * we place the generics in the signature of the function
 * where we would usually specify the data types of the parameters and return value.
 * For examples:
 *            ``find_largest<T>(list: &[T]) -> &T {...}`` (understand as "the function find_largest is generic over some type T")
 *            ``find_largest<X>(list: &[X]) -> &X {...}``
 *
 * Here, the <X> and <T> are used to parameterize the concrete datatypes (i32, f64, ...),
 * we can use any character we desire, like <Y>, <Z>, <A>, ...
 * But in Rust convention, <T> is recommended because it stands for "type"
 */

 // ------------------------------------------------------------------------------------------- //
 // -------------------------------- demo_find_largest_generic -------------------------------- //
 // ------------------------------------------------------------------------------------------- //

 fn find_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
     let mut largest = &list[0];

     for item in list {
         if item > largest {
             largest = item;
         }
     }

     largest
 }
 /*
  * In this function, we don't just use <T>,
  * we use <T: std::cmp::PartialOrd>.
  * ``std::cmp::PartialOrd`` is a ``TRAIT`` that belongs to types whose values can be partially ordered.
  * (``std::cmp::Ord`` is stricter than ``std::cmp::PartialOrd``)
  *
  * Why we have to do this?
  * Because, in the function body, we have ``item > largest``,
  * and only values of ordered types (like i32, f64, char, ...) can be compared like this.
  *
  * If we just use <T>, the compiler will panic,
  * because the comparison ``item > largest`` cannot work for all possible types <T> in Rust.
  *
  * So, this ``<T: std::cmp::PartialOrd>`` restricts the types valid for T
  * to those that implement ``PartialOrd``
  */

 fn demo_find_largest_generic() {
     let number_list = vec![34, 50, 25, 100, 65];
     let result = find_largest(&number_list);
     println!("The largest number is {result}");

     let char_list = vec!['m', 'a', 'q', 'y'];
     let result = find_largest(&char_list);
     println!("The largest char is {result}");
 }

 // ------------------------------------------------------------------------------------------ //
 // ---------------------------------- Struct with Generics ---------------------------------- //
 // ------------------------------------------------------------------------------------------ //

 #[derive(Debug)]
 #[allow(dead_code)]
 struct PointT<T> {
     x: T,
     y: T,
 } // In this struct, both x and y must have the same type

 #[derive(Debug)]
 #[allow(dead_code)]
 struct PointTU<T, U> {
     x: T,
     y: U
 } // In this struct, x and y can have different types (like x<i64> and y<f32>)

 fn demo_struct_generic() {
     let xy_integer = PointT{x: 5, y: -7};
     println!("xy_integer = {xy_integer:?}");

     let xy_float = PointT{x: 2.32451, y: -8.4523};
     println!("xy_float = {xy_float:?}");

     // let xy_different = PointT{x: 2.3, y: 5}; // This will fail
     let xy_different = PointTU{x: 2.3, y: 5};
     println!("xy_different = {xy_different:?}");
 }

 // ---------------------------------------------------------------------------------------- //
 // ---------------------------------- Enum with Generics ---------------------------------- //
 // ---------------------------------------------------------------------------------------- //
 /*
  * We have already worked with many enum whose types are generics.
  *
  * ##################################
  *
  * enum Option<T> {
  *     Some(T),
  *     None,
  * }
  *
  * The Option<T> enum is generic over type T and has two variants:
  * + ``Some(T)``, which holds one value of type T
  * + `None``, which doesn’t hold any value.
  *
  * Because Option<T> is generic, we can use it for any type.
  *
  * ##################################
  *
  * enum Result<T, E> {
  *     Ok(T),
  *     Err(E),
  *  }
  *
  * The Result<T, E> enum is generic over two types, T and E , and has two variants
  * + ``Ok(T)``, which hold a value of type T
  * + ``Err(E)``, which holds a value of type E
  *
  * This definition makes it convenient to use the Result enum anywhere we have an operation
  * that might succeed (return T) or fail (return E)
  */

  // ---------------------------------------------------------------------------------------------------- //
  // ---------------------------------- impl and methods with Generics ---------------------------------- //
  // ---------------------------------------------------------------------------------------------------- //
  // Generics can also be used with impl and methods

  ///////////////////////
  // impl and Generics //
  ///////////////////////

  #[derive(Debug)]
  #[allow(dead_code)]
  struct Point<T> {
      x: T,
      y: T,
  }

  impl<T> Point<T> { // ``impl<T>`` helps Rust understand that the T in ``Point<T>`` is a generic, not a concrete datatype
      fn x(&self) -> &T {
          &self.x // Return the value T of x when call ``some_point.x()``
      }
  }

  fn demo_impl_generic() {
      let p = Point{x: 5, y: 10};
      println!("p.x = {}", p.x())
  }

  /////////////////////////
  // method and Generics //
  /////////////////////////

  #[derive(Debug)]
  #[allow(dead_code)]
  struct Coordinate<X1, Y1> { // Now X1 and Y1 are generics
      x: X1,
      y: Y1,
  }

  impl<X1, Y1> Coordinate<X1, Y1> { // impl is defined with <X1, Y1>
      fn mixup<X2, Y2>(self, other: Coordinate<X2, Y2>) -> Coordinate<X1, Y2> { // but here, the method ``mixup()`` is defined with <X2, Y2>
          Coordinate {                                                          // this also helps Rust understand <X2, Y2> as generics, not concrete datatypes
              x: self.x,
              y: other.y
          }
          // mix the self.x and other.y into a new Coordinate instance
      }
  }

  fn demo_method_generic() {
      let coord1 = Coordinate{x: 32, y: 53.6};
      let coord2 = Coordinate{x: 3.28, y: false};

      let coord_mix = coord1.mixup(coord2);

      println!("coord_mix = ({}, {})", coord_mix.x, coord_mix.y)
  }

  /*
   * The purpose of this example is to demonstrate a situation in which
   * some generic parameters are declared with ``impl`` like ``impl<X1, Y1>``
   * and some are declared with the method definition like ``mixup<X2, Y2>()``
   *
   * Here,
   * + X1 and Y1 are declared after impl because they go with the struct definition.
   * + X2 and Y2 are declared after ``fn mixup`` because they’re only relevant to the method
   */

   // -------------------------------------------------------------------------------------------------------- //
   // ---------------------------------- Performance of Code Using Generics ---------------------------------- //
   // -------------------------------------------------------------------------------------------------------- //
   /*
    * Good news: using generic types won’t introduce any runtime cost, so it won't make the program run slower.
    * Rust accomplishes this by performing ``monomorphization`` of the code using generics at compile time.
    *
    * Monomorphization is the process of turning generic code into specific code
    * by filling in the concrete types that are used when compiled.
    *
    * The compiler looks at all the places where generic code is called and generates corresponding code for the concrete types
    *
    * Take an example with ``Option<T>``,
    * if we call ``Some(342)`` -> compiler generates ``Option<i32>``
    * if we call ``Some(String::from("Aloha")) -> compiler generates ``Option<String>``
    *
    * Because the generic codes are compiled into to codes that use specific types
    * => we pay no runtime cost for this
    */

 // ################# //
 //       main()      //
 // ################# //

 fn main() {
     println!();

     demo_find_largest_generic();

     println!("\n===================================================================\n");

     demo_struct_generic();

     println!("\n===================================================================\n");

     demo_impl_generic();
     demo_method_generic();
 }
