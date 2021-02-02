# Meeting numero uno

* cargo watch- very important dependency that lints your Rust.
  * "If cargo watch passes it should not hit any runtime panics"
* rustup- can be used to set up other toolchains to compile to other processors/microarchitectures
  * How do you compile for other microarchitectures?
* Imutable refences
  * avoids memory allocations
  * Threads/concurrency- need to share references between threads, and need to know whether that other thread has the ability to mutate the value.
