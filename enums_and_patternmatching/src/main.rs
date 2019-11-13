use std::string::ToString;

#[derive(Debug)]
pub struct Bed {
    size: i32,
    amount: i32,
}

pub struct Drawer {
    size: i32,
    amount: i32,
}

#[derive(Debug)]
pub enum Room {
    Kitchen(i32),
    Bedroom(Bed),
    Hallway(Door),
    Lounge,
}

#[derive(Debug)]
pub enum Door {
    Slideing(Lock),
    Swinging(Lock),
    Simple(Lock),
}

#[derive(Debug)]
pub enum Lock {
    Magnetcard,
    Combination,
    Key,
}

fn main() {
    use self::Room::*; // direct access to anything in Room without prefix
    use self::Door::*;
    use self::Lock::*;
    let b1 = Bed {size: 4, amount: 2};
    let b2 = Bed {size: 3, amount: 6};
    let b3 = Drawer {size: 3, amount: 6};
    // without imports this call would be:
    // let r = Room::Hallway(Door::simple(Key::magnetcard));
    let r = Hallway(Simple(Magnetcard));
    let f: i32 = 10;

    println!("This variable r contains your current room which is a: {:?} (debug print)", r);

    // let loc = match r {  // careful to always return only oly same type here
    //     Bedroom(b) => b1, // in case Bedroom return size an amount of beds
    //    Kitchen(f) => b2, // in case Kitchen return am I32
    //    // Hallway(n) => b3, // this throws due to type mismatch
    //    _ => Bed {size: 0, amount:0},
    // };
    // println!("{:?}", loc);

    if let Hallway(Door(Lock)) = r {
        println!("This hallway has a {} door with a {} lock.", Simple.ToString(), Magnetcard.ToString())
    };
}
