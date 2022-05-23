use std::collections::HashMap;
pub fn run(){
    let mut map=HashMap::new();
    map.insert(0,"Hi");
    map.insert(1,"Hi2");
    println!("{:?}",map);
    println!("{:?}",map[&0]);

    match map.get(&1){  
        Some(str)=>println!("{}",str),
        _=>println!("Doesnt exist in map"),
    }

    match map.get(&2){
        Some(str)=>println!("{}",str),
        _=>println!("Doesnt exist in map"),
    }

    map.remove(&0);
    println!("{:?}",map);
}