mod module1; // Just declare, no need definition with {}
mod module2; // Just declare, no need definition with {}
// -> Other crates can now run ``use crate::module1::something`` and ``use crate::module2::something``

use crate::module1::module1_demo::display;
use crate::module2::module2_demo;

fn main() {
    display();
    module2_demo::display();
}
