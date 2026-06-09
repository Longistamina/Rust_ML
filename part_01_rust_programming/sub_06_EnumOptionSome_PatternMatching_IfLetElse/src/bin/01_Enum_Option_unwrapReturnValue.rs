/*
 * Enums allow you to define a type by enumerating its possible variants.
 *
 * For example, we may want to say that Rectangle is one of a set of possible shapes that also includes Circle and Triangle.
 * -> Rust allows us to encode these possibilities as an enum.
 *
 * So, we could use enum when we already know the possibility of all variants.
 *
 * Enum can also has methods using ``impl`` like Struct.
 *
 * ``Option`` is an enum defined by the standard library,
 * it encodes the very common scenario in which a value could be something, or it could be nothing.
 */

 // --------------------------------------------------------------------------------------- //
 // ------------------------------------- Simple Enum ------------------------------------- //
 // --------------------------------------------------------------------------------------- //

 #[allow(dead_code)] // silence some warnings
 #[derive(Debug)]
 enum IpAddrDemo { // Defining an ``IpAddr`` enumeration with only 3 possiblities
     V4(String), // version 4 address, represented by a String
     V6(String), // version 6 address, represented by a String
     V8(u8, u8, u8, u8), // version 8 address, represented by 4 u8 values
 }

 // -------------------------------------------------------------------------------------------- //
 // ------------------------------------- Enum with method ------------------------------------- //
 // -------------------------------------------------------------------------------------------- //

 #[allow(dead_code)] // silence some warnings
 #[derive(Debug)]
 enum Message {
     Quit,
     Move {x: i32, y:i32}, // A struct inside an enum
     Write(String),
     ChangeColor(i32, i32, i32)
 }

 impl Message {
     fn call(&self) {
         println!("{self:?}")
     }
 }

 // ------------------------------------------------------------------------------------------- //
 // ------------------------------------- ``Option`` enum ------------------------------------- //
 // ------------------------------------------------------------------------------------------- //
 /*
  * ``Option`` is an enum defined by the standard library,
  * it encodes the very common scenario in which a value could be something, or it could be nothing.
  *
  * For example, if you request the first item in a non-empty list, you would get a value.
  * If you request the first item in an empty list, you would get nothing.
  * -> this concept helps compiler check whether you’ve handled all the cases you should be handling
  *
  * Not only that the features you include are important, but also the features you exclude.
  * -> null reference (the things you exclude)
  *
  * However, Rust does not have nulls,
  * but it does have an enum that can encode the concept of a value being present or absent.
  * -> That is ``Option<T>``
  */

  /*
  enum Option<T> { // <T> is a generic type parameter
      None, // No value
      Some(T) // Some values of type T
  }

  It's included in the prelude; you don’t need to bring it into scope explicitly.
  Its variants are also included in the prelude: You can use ``Some`` and ``None`` directly

  REMEMBER: ``Option<T>`` and ``T`` are two different types!!!

  Use ``Some(T).unwrap()`` to return the value typed T. If it's None, then panic
  -> Help ensure the variable is Some, not None (handle the null reference)
  */


// ############ //
// ## main() ## //
// ############ //

 fn main() {
     let home = IpAddrDemo::V4(String::from("127.0.0.1")); // Address version 4, represented by String
     let loopback = IpAddrDemo::V6(String::from("::1"));
     let work = IpAddrDemo::V8(127, 0, 1, 1); // Represented by 4 u8 values

     println!();
     println!("home addr = {home:?}");
     println!("loopback addr = {loopback:?}");
     println!("work addr = {work:?}");

     println!("\n==============================================================================\n");

     let msg_quit = Message::Quit;
     let msg_move = Message::Move{x: -32, y: 24};
     let msg_write = Message::Write(String::from("Write something here"));
     let msg_change_color = Message::ChangeColor(23, 55, 246);

     msg_quit.call();
     msg_move.call();
     msg_write.call();
     msg_change_color.call();

     println!("\n==============================================================================\n");

     let some_number = Some(5); // Type is ``Option<i32>``, not ``i32``
     let some_str = Some("Hi abc."); // Type is ``Option<&str>``, not ``&str``
     let absent_number: Option<i32> = None;

     println!("some_number = {some_number:?}");
     println!("some_str = {some_str:?}");
     println!("absent_number = {absent_number:?}");

     let x: i8 = 3;
     let y: Option<i8> = Some(5);

     // let sum = x + y;
     // -> COMPILER PANIC, because ``i8`` and ``Option<i8>`` are two different types!

     let sum = x + y.unwrap(); // Convert from Some(T) to T, then add
     println!("x + y = {sum}")


 }
