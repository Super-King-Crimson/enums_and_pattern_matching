#![allow(unused, dead_code)]
//Use if let syntax if you only want to check for one pattern of a match

//Let's say we have an attack list:
#[derive(Debug)]
enum Attack {
    //count represents what hit of the light combo the player is currently on
    LightCombo { count: u8 },
    HeavyAttack,
    //tp_mode represents whether we should teleport above a player to hit them or do the move in place
    MeteorHeel { tp_mode: bool },
    WindLash,
}

pub fn explain() {
    let atk1 = Attack::LightCombo { count: 3 };
    let atk2 = Attack::MeteorHeel { tp_mode: true };

    //These two functions do the same thing  
    print_count_match(atk1);
    print_count_if_let(atk2);
}

fn print_count_match(atk: Attack) {
    match atk {
        Attack::LightCombo { count } => println!("Current count: {count}"),
        //ewwww boilerplate code
        _ => (),
    }
}

fn print_count_if_let(atk: Attack) {
    //So clean: we only write what's necessary
    if let Attack::LightCombo {count} = atk {
        println!("Current count: {count}");
    }
}