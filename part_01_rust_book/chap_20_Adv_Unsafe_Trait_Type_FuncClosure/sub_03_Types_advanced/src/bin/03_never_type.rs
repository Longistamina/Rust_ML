/*
The never type, written `!`, represents something that never produces a value.
A function that always panics, exits, or otherwise never returns can use it as its return type

Key idea: `!` doesn’t stand for a special value.
It marks a path that cannot produce a value,
which lets Rust type-check branches that exit, panic, or continue instead.
*/

fn stop() -> ! {
    panic!("stopping");
}

fn main() {
    stop()
}
