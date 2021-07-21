### Jocelyn suggested this video by Jon Gjengset

https://www.youtube.com/watch?v=xcygqF5LVmM&t=188s

### 22 July:
I have now got to 1:02:13 - Limitation: Multiple Traits

In this episode of Crust of Rust, we go over static and dynamic dispatch in Rust by diving deep into generics, monomorphization, and trait objects. As part of that, we also discuss what exactly the Sized trait is, what it's for, and how it interacts with Dynamically Sized traits.

0:00:00 Introduction
0:03:08 Monomorphization
0:16:13 (Static) Dispatch
0:22:49 Trait Objects
0:27:13 The Sized Trait
0:39:34 Sizing Unsized Types
0:46:47 Can I Recover The Concrete Type?
0:47:56 Dynamic Dispatch
0:53:08 Vtables
1:02:13 Limitation: Multiple Traits
1:08:32 Limitation: Associated Types
1:10:30 Limitation: Static Trait Methods
1:15:48 Disallowing Trait Objects
1:20:58 Limitation: Generic Methods
1:30:53 Limitation: No Non-Receiver Self
1:33:00 Partial Object Safety
1:39:54 Dropping Trait Objects
1:43:03 Dynamically Sized Types
1:48:30 Manual Vtables in std
1:51:45 Q&A: Making Your Own DST
1:53:41 Box([u8]) vs. Vec(u8)
1:55:18 dyn Fn() vs. fn() vs. impl Fn()
2:02:00 No Coherence This Stream
2:03:00 Runtime Trait Detection
2:04:41 Double-Dereferencing dyn Fn() 
2:06:55 Unsafe Vtable Comparions
2:09:06 Slice of Trait Objects
2:10:05 Codegen Units and Vtables
2:10:55 The Any Trait

You can read more about Rust monomorphization in the Rust book at https://doc.rust-lang.org/book/ch10-0.... It also has some good discussion about Trait Objects (https://doc.rust-lang.org/book/ch17-0...) and Dynamically Sized Types (https://doc.rust-lang.org/book/ch19-0...). For more details, the Rust reference also has more details on Trait Objects (https://doc.rust-lang.org/nightly/ref...) and Object Safety (https://doc.rust-lang.org/nightly/ref...), and so does the Rustonomicon (https://doc.rust-lang.org/nomicon/exo...).

Live version with chat: https://youtu.be/TOwiCW8zXzU
Licence
Creative Commons Attribution licence (reuse allowed)