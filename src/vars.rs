pub fn run(){
    //vars hold primitive data or references to data.
    //vars are immutable by default.
    //all vars are blockscoped
    let  name="brad";   
    let mut age=20;
    println!("my name is {0} and my age is {1}",name, age);
    age=38;
    println!("my name is {0} and my age is {1}",name, age);

    //define constant
    const ID :i32=001;
    println!("ID:{}",ID);

    //assign mul variables at once

    let(_myname,_myage)=("brad",37);

    println!("{} is {}",_myname,_myage);
}