/*
================================================================================
                    RUST ERROR HANDLING GUIDELINES (SUMMARY)
================================================================================

Choose between PANIC (crashing) vs. RESULT (recoverable error) based on whether
the failure is "expected" or a catastrophic "bug/contract violation."

--------------------------------------------------------------------------------
1. WHEN TO USE `panic!` (Unrecoverable Errors)
--------------------------------------------------------------------------------
Use `panic!` when your code enters a "Bad State" (an invalid, broken, or
contradictory state that breaks guarantees). This is ideal when:

* Unexpected / Not Normal: The failure is rare and shocking (e.g., NOT a user
  typing a bad password, but a system contract completely breaking).
* Dependent Logic: Following code absolutely relies on NOT being in this bad
  state to run safely.
* Insecure or Harmful: Continuing execution would risk data corruption or
  security vulnerabilities (e.g., out-of-bounds memory access).
* Outside Your Control: External code/dependencies return a totally broken state
  that you cannot possibly patch or fix at runtime.
* Caller Bug (Contract Violation): The person using your library passed values
  that make absolutely no sense. Crashing alerts them to fix their code during
  development.

--------------------------------------------------------------------------------
2. WHEN TO RETURN `Result<T, E>` (Recoverable Errors)
--------------------------------------------------------------------------------
Use a `Result` when failure is an expected, predictable possibility of normal
operations.

* Expected Failures: Examples include malformed user input, network timeouts, or
  hitting an API rate limit.
* Caller Choice: Returning a `Result` passes the decision-making down to the
  calling code so they can choose how to handle, retry, or log the error.

--------------------------------------------------------------------------------
3. PRO-TIP: LEVERAGE THE TYPE SYSTEM (Avoid Bloated Runtime Checks)
--------------------------------------------------------------------------------
Writing dozens of `if` checks in every function is annoying and verbose. Instead,
let the Rust compiler do the validation for you before the code even runs:

* Guaranteeing Existence: Use a direct type instead of an `Option` if a value is
  strictly mandatory. If someone tries to pass nothing (`None`), it won't compile.
* Guaranteeing Domains: Use unsigned integers like `u32` if a value can never
  be negative. The compiler blocks negative inputs at compile time.

================================================================================
                           QUICK DECISION MATRIX
================================================================================
 Is the problem expected/likely?  --> Return Result<T, E> (e.g., File Not Found)
 Is it a developer bug/exploit?   --> Call panic!       (e.g., Array Out-of-Bounds)
 Can it be blocked beforehand?    --> Use Strict Types  (e.g., u32 instead of i32)
================================================================================
 */

 fn main() {}
