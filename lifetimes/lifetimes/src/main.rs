fn main() {
    what_is_a_lifetime();
    let string1 = String::from("long string is long");
    let result;
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());

    let firstword = first_word(string1.as_str());
    println!("The longest string is {}", result);
    println!("this is first word {}", firstword);
    hello_there();
}


fn what_is_a_lifetime(){
    let r;

        let x = 5;
        r = &x;

    println!("r: {}", r);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len(){
        x
    } else {
        y
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn hello_there(){
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }

    println!("The longest string is {}", string1);
}