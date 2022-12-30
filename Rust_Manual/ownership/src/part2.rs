pub fn part_2() {
    println!("*********** PART 2 ***********");

    slice_type();
}

fn next_topic() {
    println!("------------------------------");
}

fn slice_type() {
    /* Slices let you reference a contiguous sequence of elements in a collection
       rather than the whole collection. A slice is a kind of reference, so it does not have ownership. */

    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();

    println!("Word = {}", word);

    next_topic();

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{hello}");
    println!("{world}");

    next_topic();

    let string = String::from("Hello World");
    let s = find_the_first_word(&string);
    println!("{}", s);
    
    next_topic();

    let a = [0, 1, 2, 3, 4];
    let slice_a = &a[..=2];

    for elem in a {
        println!("{elem}");
    }

    next_topic();
    for elem in slice_a {
        println!("{}", elem);
    }

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}

fn find_the_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s;
}