/*
 * In Rust, a struct (short for "structure") is a custom data type
 * that allows you to group multiple related values of different types together into a single cohesive unit.
 *
 * Like tuples, the pieces of a struct can be different types.
 * Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean.
 *
 * So, you can use the names to access the corresponding data
 * -> structs are more flexible than tuples
 */

// ---------------------------------------------------------------- //
// --------------------- Define a struct -------------------------- //
// ---------------------------------------------------------------- //

struct User { // Use "struct" keyword to define a struct named "User"
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
    // active, username, email, sign_in_count are called "fields" of struct
    // Must declare values for all fields
}

////////////
// main() //
////////////

fn main() {
    // ------------------------------------------------------------------- //
    // --------------------- Immutable instance -------------------------- //
    // ------------------------------------------------------------------- //

    let user_1 = User { // initiate an immutable instance user_1 using struct User
        active: true,
        email: String::from("longistamina@email.com"),
        sign_in_count: 1,
        username: String::from("longistamina"),
        /*
         * Use "key: value" to assign values to struct's fields/keys
         * We don’t have to specify the fields in the same order in which we declared them in the struct
         *
         * Since user_1 is immutable, once all its fields are declared, they are unchangeable
         */
    };

    println!("user_1 username = {}", user_1.username); // use instance.field to access the value of that field
    println!("user_1 email = {}", user_1.email);
    println!("user_1 active = {}", user_1.active);
    println!("user_1 sign_in_count = {}", user_1.sign_in_count);
    println!("===============================================================================");

    // ------------------------------------------------------------------ //
    // --------------------- Mmutable instance -------------------------- //
    // ------------------------------------------------------------------ //

    let mut user_2 = User { // declare an mutable instance "user_2" using "mut"
        username: String::from("hanah"),
        email: String::from("hanah@email.com"),
        active: false,
        sign_in_count: 2,
    };
    // the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.

    println!("user_2 email (original) = {}", user_2.email);

    user_2.email = String::from("flower@google.com"); // modify user_2 email (since it's mutable)
    println!("user_2 email (mutated) = {}", user_2.email);
    println!("===============================================================================");

    // -------------------------------------------------------------------- //
    // ------------------------- Init shorthand --------------------------- //
    // -------------------------------------------------------------------- //

    fn build_user(username: String, email: String) -> User {
        User {
            active: true,
            username, // equivalent to ``username: username``
            email, // equivalent to ``email: email``
            sign_in_count: 1,
            /*
             * since the parameter names and the struct field names are exactly the same,
             * we can use the ``field init shorthand`` syntax to rewrite ``build_user``
             * so that it behaves exactly the same but doesn’t have the repetition of ``username`` and ``email``
             * NOTE: since we don't use ``key: value``, the order must be respected
             */
        }
    }

    let user_3 = build_user(String::from("hadilao"), String::from("hadilao@sh.com"));
    println!("user_3 username = {}", user_3.username);
    println!("user_3 email = {}", user_3.email);
    println!("===============================================================================");

    // --------------------------------------------------------------------------------------- //
    // ------------------------- new struct by Updating old struct --------------------------- //
    // --------------------------------------------------------------------------------------- //

    let user_4 = User {
        email: String::from("hdl2@gmail.com"),
        ..user_3 // Use all remain fields except ``email`` from ``user_3`` to create user_4
        /*
         * ``..user_3`` is called ``struct update syntax``
         * This is even more convenient than using the ``build_user()`` function
         */
    };

    println!("user_4 username = {}", user_4.username); // same as user_3
    println!("user_4 email = {}", user_4.email);
    println!("user_4 active = {}", user_4.active);

    // println!("user_3 username = {}", user_3.username)
    /*
     * NOTE: after using ``..user_3`` to update (or create ``user_4``),
     *       the field ``username`` of the ``user_3`` is MOVED into user_4,
     *       meaning that its memory location are freed, cannot be borrowed or referenced anymore!!!!!
     *      (why? because field ``username`` is a String Type, its content is on the heap and does not have copy trait)
     * -> ``println!("user_3 username = {}", user_3.username)`` will cause compiler panic
     */

     println!("user_3 email = {}", user_3.email);
     println!("user_3 active = {}", user_3.active)
     /*
      * ``user_3.email`` field is still remained, because it was not used to update/create user_4
      * (in other word, this field was not moved, so it can still be borrowed or referenced)
      *
      * But ``user_3.active`` field is also used to update/create ``user_4``, why is it still remained?
      * Because, ``user_3.active`` is an immutable and fixed-size object, its content is stored on the stack
      * -> and it has COPY TRAIT, that's why it's still remained, unlike ``user_3.username``
      */
      println!("===============================================================================");

      // --------------------------------------------------------------------------------------- //
      // ------------------------------------- Tuple Structs ----------------------------------- //
      // --------------------------------------------------------------------------------------- //
}
