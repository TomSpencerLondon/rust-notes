# Meeting 2

## Chapter 3 sections 1-2, Alisdair presenting

* Live coding session from github repo: https://github.com/midmandle/rust-book-chapter-3
* Cannot shadow a const
* `char.toUppercase` always returns a string because in some languages a lowercase character might uppercase to multiple characters
  * Example is a character in German that is one S-like char in lowercase, but in uppercase becomes two characters like "SS"
* Why do tuples use `.1` instead of `[1]`?
  * To make it obvious to the reader that it is a tuple, not an array. You can tell just by looking at where it's consumed because they're distinct
* Discussion about tuples & type systems
  * I asked what the appropriate usecases are for tuples, argued that they can reduce the effectiveness of the type system- eg `(1,2,3)` could be R,G,B or Day,Month,Year, or x,y,z. Without a type you can't be sure that what you're receiving is the type of data you expect
  * Jorge argued in favour of using Tuples, we went back and forth a bit with most other people adding their 2c. 
  * It's 
  * Jocelyn- there will be some more usages for tuples later involving enums that will make their value clearer

## Chapter 3 sections 3-5, Ivan presenting

* Separation between statement and expression
  * Jorge doesn't like this. Feels it's complicating things too much- putting a semicolon in shouldn't break your code.
  * Assigning a value does not return an expression
  * Semicolon says "something else is coming"; good way to think about it.
  * Reason for doing it: Probably performance-related. Because statements don't return values you don't create anything in memory when you execute a statement and discard it's return value.
* You can include parenthesis for boolean expressions, they are optional
* `while index < x.len()` vs `for element in x.iter()`
  * `.iter()` is used for lots of other stuff later and it's good to get used to doing it

  * It's more performant. `x[index]` requires doing a boundary check `index` times, `for` doesn't.
* Inclusive ranges- `(1..=4)` is 1-4, whereas `(1..4)` is 1-3
* Explicitly define types with values- `let n = 12u8` means "value twelve of type unsigned 8-bit integer", the type of `n` is then ineferred to be the same.
* Convert between scalars- `12 as u8`, like casting.