#![allow(unused, dead_code)]
use std::marker::PhantomData;

use rand::Rng;

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    Dollar,
}

fn get_25u8() -> u8 {
    25
}

//A match statement is like a coin sorter: the first hole that fits is what the control flow will 'fall into'
//It's main difference vs. if is that we don't have to have the statement be a bool (so we can return types)
//The combination of pattern to match + code expression is called an arm
impl Coin {
    fn get_value(&self) -> u8 {
        match self {
            Coin::Penny => 1,

            //here's the pattern: is this coin (self) a nickel?
            Coin::Nickel => {
                //here's the code: print this message, and return the u8 value 5
                println!("Do people even use these?");
                5
            }
            
            //here's the pattern
            Coin::Dime => 10,  //and the expression is just 10

            //If you only have one line of code, you don't need curly braces, just drop a comma at the end
            Coin::Quarter => get_25u8(),

            //but if you have more than one, you need curly braces (and comma becomes optional)
            Coin::Dollar => {
                println!("Is that you, Patrick?");
                //and don't forget to return a value
                100
            },
        }
    }
}

pub fn explain() {
    let coin = Coin::Nickel;
    let val = coin.get_value();

    println!("This coin's value is {} {}.",
    val,
    if val == 1 { "cent" } else { "cents" });

    match_binding();
}


//Match statements can 'bind' to parts of the value that match the pattern
//Let's make enum to represent cards in a deck that aren't numerical:
#[derive(Debug)]
enum Suit { Clubs, Spades, Hearts, Diamonds, }
#[derive(Debug)]
enum JokerSuit { FullColor, OneColor, }

enum FaceCard {
    Jack(Suit),
    Queen(Suit),
    King(Suit),
    Ace(Suit),

    //Jokers don't have a traditional suit
    Joker(JokerSuit),
}

impl FaceCard {
    fn announce_type_return_value(&self) -> u8 {
        match self {
            FaceCard::Jack(suit) => {
                println!("Jack of {:?}.", suit);
                11
            }

            FaceCard::Queen(suit) => {
                println!("Queen of {:?}.", suit);
                12
            }

            FaceCard::King(suit) => {
                println!("King of {:?}.", suit);
                13
            }

            FaceCard::Ace(suit) => {
                println!("Ace of {:?}.", suit);
                14
            }

            FaceCard::Joker(suit) => {
                println!("{:?} Joker.", suit);
                0
            },
        }
    }
}

fn match_binding() {
    //The match statement will bind to the Suit enum of the card we pass in, and retrieve it
    println!("\n------- Let's see your cards. -------");

    let my_cards: [FaceCard; 3] = [
        FaceCard::Joker(JokerSuit::OneColor),
        FaceCard::Ace(Suit::Spades),
        FaceCard::Jack(Suit::Clubs)
    ];

    let mut my_values: [u8; 3] = [255, 255, 255];

    for i in 0..my_cards.len() {
        my_values[i] = {
            println!("The identity of card {} is:", i + 1);
            my_cards[i].announce_type_return_value()
        };

        println!();
    }

    println!("Their values in order are:");
    for value in my_values {
        println!("{}", value)
    } 

    println!("");
    option_matching();
}


fn option_matching() {
    //Now that we know match statements can get inner values out of an enum, we can unpack Options!
    let rng: u8 = rand::thread_rng().gen_range(0..3);

    let val: Option<u8> = {
        if rng == 0 {
            add_to_some(None)
        } else if rng == 1 {
            add_to_some(Some(0))
        } else {
            add_to_some(Some(235))
        }
    };

    println!("{val:?}");
    
    catch_alls();
}


//Accepts a u8 option, adds 20 to it if Some, returns nothing if None
fn add_to_some(some_none: Option<u8>) -> Option<u8> {
    match some_none {
        None => None,
        Some(num) => Some(num + 20),
    }
    //Also, match is EXHAUSTIVE: we must have code for all possible values of some_none, or it won't compile
    //Try commenting out one of the arms and see what the compiler says
}


fn catch_alls() {
    //There are two catch_alls: _ and variable coverage (idk what its real name is)

    //Let's say we have a board game: 
    //6 means you roll again and go double those spaces forward, 
    //1 means you roll again and go that many spaces back,
    //anything else you just go that many spaces forward

    //use variable coverage if you want to use the value you catch
    let roll: u8 = 3;
    
    match roll {
        6 => println!("Roll again, move double!"),
        1 => println!("Roll again, move backwards!"),
        other => println!("Move forward {other} spaces!"),
    }
    //This is okay, other catches all patterns not explicitly listed
    //the catch all variable can be basically anything btw not just other (its not a keyword)

    //Use _ if we aren't going to use the value we catch
    match roll {
        6 => println!("Roll again, move double!"),
        1 => println!("Roll again, move backwards!"),
        _ => println!("Move forward, idc how many spaces"),
    }

    //Make sure to put catchalls LAST: rust evaluates arms in order, so a catch all will block other conditions

    //Let's change some rules:
    match roll {
        6 => println!("Move forward a space."),
        1 => println!("Move backward a space!"),
        3 => println!("Move another player forward or backward a space."),
        //Nothing happens if you don't roll a 6, 1, or 3, so we return a unit value (empty tuple)
        _ => (),
    }

    println!();
    match_ownership();
}


//Use references if you want to refer to your match's inner contents
fn match_ownership() {
    let mut heaped_option: Option<Vec<String>> = Some(vec![String::from("Hello"), String::from("World")]);

    //this is ok: we don't ever take ownership of stacked_option's inner data, we just check if it exists or not
    match heaped_option {
        Some(_) => println!("Yup, stacked"),
        None => println!("Not stacked"),
    }
    println!("{heaped_option:?}");

    //this is not: we took ownership of the heap data, then it went out of scope
    // match heaped_option {
    //     Some(data) => println!("{}", data[1]),
    //     None => (),
    // } 

    //heaped_option lost ownership of heap data, so we can't use it anymore
    // println!("{heaped_option:?}");

    //to fix, make the match use a reference:
    let option_ref = &mut heaped_option;

    match option_ref {
        Some(data) => {
            data[0] = String::from("Goodbye");
            data[1] = String::from("Earth");
        }
        None => (),
    } 

    let heap = heaped_option.unwrap();

    println!("{} {}", heap[0], heap[1]);

    // *** Rust 'pushes down' references from outer to inner fields in match statements, 
    //so we now have &Option<&Vec<&String>> and ownership isn't taken
    //Check out rust binding modes for more info
}