//
// let s : &str = "Hello World"; // is a "string literal", a.k.a. a "slice"
//


fn first_word_000(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// fn first_word(s: &String) -> &str {	// String is "heaped" string data
fn first_word(s: &str) -> &str {	// str is a *char; use for parsing string data
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let s : &str = "Hola!" ;
    println!("{}", s);

    let s = String::from("I'm OK, how are you doing today?") ;
    let ll : usize = first_word_000(&s) ;
    println!("{} is {} characters long",s,ll ) ;

    let sl = first_word(&s[8..16]) ;
    println!("first part of {}, is {}", s, sl) ; // this works. s still has R permissions

    // test question:
    let mut s = String::from("hello");
    // for (i, &item) in s.as_bytes().iter() {
    //    if item == b'l' {
    //        // s.push_str(" world"); // s can't have W access here b/c immutable borrow in iter()
    //        println!("{}",s[..]);
    //    }
    // }

}
