////////// FUNCTIONS //////////

// 'main' is the entry point
// functions use the 'fn' keyword
fn main() {
    println!("==> main");

    another_function();

    print_int(10);

    print_2_ints(11, 12);

    statement_vs_expression();

    println!("==> five: {}", five());

    println!("==> plus_one: {}", plus_one(5));

    basic_if();

    else_if();

    if_expression();

    infinite_loop();

    break_infinite_loop();

    while_loop();

    loop_over_collection();

    for_loop();

    for_with_range();
}

// use snake case
// fns can be defined before or after main
// scope
//      - starts with {
//      - ends with }
fn another_function() {
    println!("==> another_function");
}

// parameters are passed inside of ()
// you must declare the parameter type in fn signatures
fn print_int(x: i32) {
    println!("==> print_int: {}", x);
}

// pass multiple parameters
// parameters do not have to be of the same type
fn print_2_ints(x: i32, y: i32) {
    println!("==> print_2_ints: {}", x);
    println!("==> print_2_ints: {}", y);
}

// statements - no return values and use ;
//      - fn definitions are statements
// expressions - return a value and do not use ;
fn statement_vs_expression() {
    // statements do not return values
    // i.e., you cannot assign them to a variable
    // let x = (let y = 6);

    // you cannot do something like that either
    // since assigning a value does not retun the value
    // x = y = 6

    // uncomment
    // let x = 5; // statement

    let y = {
        let x = 3; // statement
        x + 1 // expressions do not have ending ;
    }; // this whole block of {} is an expression
    // calling a macro is an expression

    println!("==> statement_vs_expression: {}", y);
}

// return value type must be specified
// the final expression is returned
// the return keyword can also be used
// the return keyword can be used to return early
fn five() -> i32 {
    // return 3;
    5
}

fn plus_one(x: i32) -> i32 {
    // adding a ; will result in an error
    // since () is a statement and not i32
    x + 1
}

////////// COMMENTS //////////

// a '//' extends a comment until the end of the line

// multiline
// comments
// require
// a '//'
// on each line

////////// CONTROL FLOW //////////

fn basic_if() {
    let number = 3;

    // {} - called arms
    // condition must be pool
    // if number { // this will not work
    if number < 5 {
        println!("==> basic_if: condition was true");
    } else {
        println!("==> basic_if: condition was false");
    }
}

fn else_if() {
    let number = 6;

    if number % 4 == 0 {
        println!("==> else_if: number is divisible by 4");
    } else if number % 3 == 0 {
        println!("==> else_if: number is divisible by 3");
    } else if number % 2 == 0 {
        println!("==> else_if: number is divisible by 2");
    } else {
        println!("==> else_if: number is not divisible by 4, 3, or 2");
    }
}

fn if_expression() {
    let condition = true;
    // if is an expression, so it can be used
    // on the right side of a let statement
    let number = if condition { 5 } else { 6 };

    // the expressions in the arms must be
    // of the same type, so this is not valid
    // because Rust must know what type a variable
    // is at compile time
    // let number = if condition { 5 } else { "six" };

    println!("==> if_expression: {}", number);
}

// ctrl-c breaks an infinite loop
fn infinite_loop() {
    let mut counter = 0;
    loop {
        counter += 1;
        println!("==> infinite_loop: {}", counter);

        if counter == 5 {
            break;
        }
    }
}

// the break keyword breaks a loop
fn break_infinite_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            // values after break are returned
            break counter * 2;
        }
    };

    println!("==> break_infinite_loop: {}", result);
}

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("==> while_loop: {}", number);
        number -= 1;
    }

    println!("==> while_loop: over");
}

fn loop_over_collection() {
    let a = [10, 20, 30, 40, 50];

    let mut index = 0;
    // error prone because the program might
    // panic if the index length is incorrect
    // also, doing a[index] is slow because the compiler
    // needs to perform this check at runtime on
    // every index
    while index < 5 {
        println!("==> loop_over_collection: {}", a[index]);
        index += 1;
    }
}

// a better solution to loop_over_collection()
fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("==> for_loop: {}", element);
    }
}

fn for_with_range() {
    // starts from 1 and ends before 4, i.e., at 3
    // this is inclusive of the end value - (1..=4)
    // rev() reverses the range
    for number in (1..4).rev() {
        println!("==> for_with_range: {}", number);
    }
    println!("==> for_with_range: over");
}