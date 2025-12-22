use std::{collections::HashMap,io};

fn name() -> String {
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("invalid");

    name.trim().to_string()
}
struct Finger {
    left:i8,
    right:i8,
}
fn main () {
    let user = name();
   println!("\n\n\n\n\n\n\n\n\n");

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

    if let Some(Finger) = derb.get(&user) {
        println!("{} 3ando {} sb3in flimna  o lisra {}"
        ,user, Finger.left, Finger.right);
        println!("trex w9");

    } else {
        println!("not included yet");
    }


}
