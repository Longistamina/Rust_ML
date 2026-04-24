/*
In the case of a string literal (&str), we know the contents at compile time, 
so the text is hardcoded directly into the final executable.
This is why string literals are fast and efficient.
But these properties only come from the string literal’s immutability.

However, for String Type, we do not know the size of the contents in advance,
we can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile,
and whose size might change while running the program.

So with the String Type, to support a mutable, growable piece of text,
we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents.

This means:
+ First, at runtime, an allocator will have to request an amount of memory to store the String with unknown size.
+ Second, When we are done with this String, we have to find a way to return back that memory (to avoid memory leak)
*/

fn main() {

}