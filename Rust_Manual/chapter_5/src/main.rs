struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/* Tuple structs have the added meaning the struct name provides but don’t
   have names associated with their fields; rather, they just have the types of the fields.  */
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/* Unit-Like Structs Without Any Fields */
struct AlwaysEqual;

fn main() {
    println!("********* 5.1 ********* ");

    let user1:User = build_user(String::from("abcd@gmail.com"), String::from("xBeast23"));

    /* Using struct update syntax */
    let user2:User = User {
        email: String::from("another@example.com"),
        ..user1
    };
    /* ..user1 must come last to specify that any remaining fields should get
       their values from the corresponding fields in user1 */

    /* Note that the struct update syntax uses = like an assignment
       We can no longer use user1 after creating user2 because the String
       in the username field of user1 was moved into user2 */

    let black:Color = Color(0, 0, 0);
    let origin:Point = Point(0, 0, 0);

    println!("origin.x = {}", origin.0);

    /* Unit-like structs can be useful when you need to implement a trait on some 
       type but don’t have any data that you want to store in the type itself.  */
    let subject:AlwaysEqual = AlwaysEqual;

    area_of_rectangle();
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

/* All functions defined within an impl block are called associated
   functions because they’re associated with the type named after the impl */
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

/* Associated functions that aren’t methods are often used for constructors
   that will return a new instance of the struct. These are often called new,
   but new isn’t a special name and isn’t built into the language. This can be 
   considered a constructor method */
impl Rectangle {
    fn square(square: u32) -> Self {
        Self {
            width: square,
            height: square,
        }
    }
}

fn area_of_rectangle() {
    println!("********* 5.2 ********* ");
    let width:u32 = 20;
    let height:u32 = 20;

    let area:u32 = area(width, height);
    println!("Area of the rectangle is {}", area);

    let mut rect:Rectangle = Rectangle {
        height: 20,
        width: 25,
    };

    let area:u32 = area_with_structs(&rect);
    println!("Area of the rectangle is {}, with the dimensions {:?}", area, rect);

    dbg!(&rect);

    rect.height = 10;
    rect.width = 5;
    let area:u32 = rect.area();
    println!("Area of the rectangle is {}, with the dimensions {:?}", area, rect);

    let rect:Rectangle = Rectangle::square(3);
    println!("Area of the rectangle is {}, with the dimensions {:?}", rect.area(), rect);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_structs(rectangle: & Rectangle) ->u32 {
    rectangle.height * rectangle.width
}