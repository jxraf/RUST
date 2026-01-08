use std::{collections::HashMap,io};

fn name() -> String {
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("invalid");

    name.trim().to_string()
}
fn number() -> i8 {
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("invalid");
    number.trim().parse().expect("please enter a valid number")
}
struct Finger {
    left:i8,
    right:i8,
}
fn main () {
    let mut derb: HashMap<String, Finger> = HashMap::new();
    
    derb.insert(
        "saaida".to_string(),
        Finger {left:6, right:6},
    );
    derb.insert(
        "hamid".to_string(),
        Finger{left:6, right:5}
    );
    derb.insert(
        "aziz".to_string(),
        Finger{left:5, right:5}
    );
    println!("enter one of BOSTA family name.");
    loop {

        let user = name();
        if user == "exit" {
            break;  
    };
    println!("\n\n\n\n\n\n\n\n\n");
    
 
    if let Some(finger) = derb.get(&user){
        println!("{} 3ando {} sb3in flimna  o {} lisra "
        ,user, finger.left, finger.right);
        if continue == yes {
    break;
        } else {

    } else {
        println!("not included yet");
        
        println!("HOW MANY FINGER IN THE RIGHT HAND");
        let right = number();
        println!("HOW MANY FINGER IN THE LEFT HAND");
        let left = number();

        derb.insert(user, Finger{left, right});
        println!("ADDED SUCCESSFULL");
        println!("\n\n\n\n\n");
        
    } } 
}
    
    

}
