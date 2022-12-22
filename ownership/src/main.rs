mod part2;


/** Ownership 
 * Rust doesn't have a garbage collector
 * Memory is managed through a system of ownership with a set of rules that the compiler checks.
 * If any of the rules are violated, the program won't compile.
 * 
 * Ownership rules:
 *  Each value in RUst has an owner
 *  There can only be one owner at a time
 *  When the owner goes out of scope, the value will be dropped
*/
fn main() {
    println!("Hello, world!");

    let mut s = String::from("hello");
    println!("{s}");

    s.push_str(", world!"); /* push_str() appends a literal to a String */

    println!("{}", s);

    /* Variables and Data Interacting with Move */
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    let s1 = String::from("This is the content of string 1");
    let s2 = s1; /* After the copy is done, rust considers s1 as no longer valid so it will drop it */

    println!("{}", s2);

    /* We can use the clone method to make a copy of the variable */
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    /* Data stored on stack do not get discarded after a copy is performed */

    let s = String::from("string");     // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                                // ... and so is no longer valid here
    let x = 5;                             // x comes into scope

    makes_copy(x);                 // x would move into the function,
                                                // but i32 is Copy, so it's okay to still
                                                // use x afterward

    let s1 = gives_ownership();         // gives_ownership moves its return value into s1
    println!("{}", s1);

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);      // s2 is moved into
                                                            // takes_and_gives_back, which also
                                                            // moves its return value into s3

    println!("{}", s3);

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    let mut s1 = String::from("hello world!");
    let len = calculate_length_2(&mut s1);

    println!("The length of '{}' is {}.", s1, len);

    /* You can not have multiple references to a value */
    /* This code won't compile */
    // let r1 = &mut s1;
    // let r2 = &mut s1;
    // println!("{}, {}", r1, r2);

    /* It avoids a data race condition */
    /*
     * A data race is:
     *  Two or more pointers access the same data at the same time.
     *  At least one of the pointers is being used to write to the data.
     *  There’s no mechanism being used to synchronize access to the data.
     */

     /* We can have multiple IMMUTABLE references to a value.
        At any given time, you can have either one mutable reference or any number of immutable references.
        References must always be valid. */
        
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    
    let r3 = &mut s; // no problem
    println!("{}", r3);

    part2::part_2();

}   // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.

/* Ownership and Functions */

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    return some_string;
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

    return a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    return (s, length)
}

fn calculate_length_2(s: &mut String) -> usize { // s is a reference to a String
    s.push_str("string");
    return s.len();
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.