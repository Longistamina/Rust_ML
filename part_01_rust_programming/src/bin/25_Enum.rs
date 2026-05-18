/*
 * Enums allow you to define a type by enumerating its possible variants.
 *
 * For example, we may want to say that Rectangle is one of a set of possible shapes that also includes Circle and Triangle.
 * -> Rust allows us to encode these possibilities as an enum.
 *
 * So, we could use enum when we already know the possibility of all variants.
 */

 enum IpAddrKind { // Defining an ``IpAddrKind`` enumeration with only 2 possiblities
     V4, // version 4 address
     V6 // version 6 address
 }

 struct IpAddr {
     kind: IpAddrKind,
     address: String
 }

 fn main() {
     let four = IpAddrKind::V4; // Create instance for version 4 IP address
     let six = IpAddrKind::V6;

 }
