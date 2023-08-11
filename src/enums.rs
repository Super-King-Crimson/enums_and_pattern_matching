#![allow(unused, dead_code)]

use crate::option_is_cool;

pub fn introduce() {
    println!(
    "      -----------Chapter 6: Enums and Pattern Matching-----------
    Enums allow you to make your own TYPES by ENUMERATING their possible variants.

    First, we'll learn how to create an enum.
    
    Then, we'll learn about Option, an enum that expresses that a value is either Something or Nothing.

    After, we'll learn about pattern matching with match, which makes it easy to map certain blocks of code
    to a certain value in an enum.

    Finally, we'll go over the if let construct, which will make enums easier to handle in code.");
}

//Enums allow you to say one value is a number of a specific group of values.
//For example, let's create an enum that represents IP Addresses
enum IpAddrKind {
    //convention is the same as structs
    V4,
    V6,
}
//Each IP address can only be a Version 4 or Version 6,
//but they're both fundamentally IPs, so they can be treated similarly

pub fn explain() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    //Note how both use :: syntax with the IpAddrKind namespace
    //Since they both use the IpAddrKind namespace, we can create a function that accepts any member of that enum
    route(four);
    route(six);
    //route(4u64);

    //Ok, this is cool, but we don't know how to represent the actual IP adress
    //While we could use a struct (with an IpAddrKind and String field to represent kind and address),
    //instead we could put the data directly into the enum with parentheses

    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    //We can attach extra data to an enum directly

    //Also, the enum definition is a function! It accepts arguments and returns an instance of the enum
    //We can also give the same members of an enum different types

    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home2 = IpAddr2::V4(127, 0, 0, 1);
    let loopback2 = IpAddr2::V6(String::from("::1"));

    implement();
}

fn route(ip: IpAddrKind) {
    //TODO: implement
}

//Here's how Rust's standard library defines IP Addresses using enums and structs:
struct Ipv4Addr {
    //stuff
}

struct Ipv6Addr {
    //more stuff
}

enum IpAddr {
    //You can attach any data to an enum, even another enum!
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

//Look at all these different fields for a message!
enum Message {
    //Woah, named fields!
    Moving {
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        speed: i8,
    },
    Warning(String),
    Multicolor(u32, u32, u32),
}

//One more thing that makes enums similar to structs is the impl block:
impl Message {
    fn display(&self) {
        //TODO: implement
    }
}

fn implement() {
    let warn = Message::Warning(String::from("Failed to touch grass"));
    let mover = Message::Moving {
        x1: -100,
        y1: -100,
        x2: 100,
        y2: 100,
        speed: 20,
    };
    let ip = IpAddrKind::V4;

    //both can call display!
    warn.display();
    mover.display();

    //this can't tho.
    //ip.display();

    option();
}

//Rust doesn't have null, but they do have a way to represent a value as existing or not existing
//Here it is: the Option enum!
enum BasicallyOption<T> {
    None,
    Some(T),
}

fn option() {
    let some_number: Option<u8> = Some(5);
    let some_char: Option<char> = Some('a');
    let absent_number: Option<i32> = None;

    //I don't really understand why but apparently null is bad and None is better
    //Here's an example of why (i think?)

    //let sum: u8 = 20 + some_number;

    //Rust doesn't know how to add a Some<u8> to a u8, so you have to convert it to a u8
    //This fixes the problem because now you can't assume something is/isn't null
    //We can use MATCH STATEMENTS (next lesson!) to handle all the values of an enum & unpack Option<T>

    //You know what? Option is so useful let's have a side lesson to talk about it
    //GO LEARN MATCH STATEMENTS FIRST THOUGH!
    option_is_cool::enumerate_coolness();
}
