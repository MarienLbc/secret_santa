use std::collections::HashMap;
use rand::Rng;

fn main() {
    let mut participants = HashMap::new();

    participants.insert(1,"Leia".to_string());
    participants.insert(2,"Obiwan".to_string());
    participants.insert(3,"Luke".to_string());
    participants.insert(4,"Yoda".to_string());
    
    let mut participants_bis = participants.clone();

    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0, 5));


    println!("{:?}",participants_bis.get(&rng.gen_range(0, 4)));

    // for (k, v) in &participants {
    //     participants.remove(v)

    // }

    // participants_bis.remove(k);

    for (l, w) in &participants_bis {
        println!("{}: \"{}\"", l, w);       
    }    

}
