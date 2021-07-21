pub trait Hei {
    fn hei(&self);
}

impl Hei for &str {
    fn hei(&self) {
        println!("hei {}", self);
    }
}

pub fn say_hei_static<H>(s: H) {
    s.hei();
}

pub fn say_hei_static_str(s: &str) {
    s.hei();
}

pub fn say_hei(s: &dyn Hei) {
    // &dyn Hei
    // stored in &
    // 1. A pointer to the actual, concrete, implementing type
    // 2. a pointer to a vtable for the referenced trait

    // What is a vtable?
    // dyn Hei, vtable:
    // struct heiVtable {
    //      hei: *mut Fn(*mut ()),
    // }
    // &str -> &dyn Hei
    // 1. Pointer to str
    // 2. &HeiVtable {
    //    hei: &<str as Hei>::hei // line 6
    // }
    s.hei();
    // s.vtable.hei(s.pointer)
}



//
// pub fn foo(h: Hei) {}
//
// struct Foo<T> { s: [u8] }

pub fn strlen<S: AsRef<str>>(s: S) -> usize {
    s.as_ref().len()
}

pub fn strlen_str(s: String) -> usize {
    s.len()
}

pub fn strlen_dyn2(s: Box<dyn AsRef<str>>) -> usize {
    s.as_ref().as_ref().len()
}


pub fn strlen_dyn(s: &dyn AsRef<str>) -> usize {
    s.as_ref().len()
}

pub fn main() {
    let x: Box<dyn AsRef<str>> = Box::new(String::from("hello"));
    strlen_dyn2(x);

    let y: &dyn AsRef<str> = &"world";
    strlen_dyn(y);

    let random = 4;
    if random == 4 {
        say_hei(&"hello");
    } else {
        say_hei(&String::from("world"));
    }
}

//
// pub fn foo() {
//     bar(&["J", "Jon"]);
//     bar(&[String::from("J"), String::from("Jon")]);
//     bar(&["J", String::from("Jon")]);
// }
//
// pub fn bar<H: Hei>(s: &[dyn Hei]) {
//     for h in s {
//         h.hei();
//     }
// }



// Dynamically sized types