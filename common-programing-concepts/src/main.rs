/* constants should use capital letters */
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    /* We can shadow variables with different types */
    let spaces = "   ";
    let spaces = spaces.len();

    println!("Nb of spaces = {spaces}");

    /* Chars */
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    /* Touple Type */
    /* 
        A tuple is a general way of grouping together a number of 
        values with a variety of types into one compound type. Tuples have 
        a fixed length: once declared, they cannot grow or shrink in size.
    */

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    println!("First element: {}, second element: {}, third element: {}", tup.0, tup.1, tup.2);

    /* Arrays */
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; /* The array named a will contain 5 elements that will all be set to the value 3 initially. */

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    println!("First element of the array: {first}, second elemenet: {second}");
    another_function(32);
    let x = five();
    println!("Value of x is = {x}");

    let number = 3;
    if number < 5 {
        println!("Condition TRUE");
    } else {
        println!("Condition FALSE");
    }

    /* Must be explicit and always provide if with a Boolean as its condition. */
}

/* In function signatures, you must declare the type of each parameter. */
fn another_function(x: u32){
    println!("This is another function, with the argument x = {x}!");

    let y = {
        let z = 3;
        z + 1
    };
    /* Expressions do not include ending semicolons. If you add a semicolon to 
       the end of an expression, you turn it into a statement, and it will then not return a value. */

    println!("The value of y is: {y}");
}

fn five() -> u32 {
    return 5;
}