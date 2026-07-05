/*
 * ``panic!()`` is a Rust macro used to panick the program/compiler
 * -> results in Unrecoverable Errors.
 *
 * There are two ways to cause a panic in practice:
 * + by taking an action that causes our code to panic (such as accessing an array past the end)
 * + or by explicitly calling the ``panic!()`` macro
 */

fn demo_panic_macro() {
    panic!("crash and burn!!!!") // Use ``panic!("message")`` to explicitly panic the program
}

fn demo_panic_action() {
    let v = vec![1, 2, 3];
    v[99]; // This panics the program because index 99 is out of bound
}

fn main() {
    demo_panic_macro();
    demo_panic_action();
}

// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// Example: `RUST_BACKTRACE=1 cargo run`
