pub fn run(){
    println!("hello from print.rs file");

    println!("{} is a {} bad","boy","gene");

    //positional arguments
    println!("{0} is from {1} and {0} likes to {2}","brad","mars","genotype");//positional arguments

//named arguments

println!("{name} likes to play {activity}",name="john",activity="baseball");//named arguments

//placeholder traits

println!("binary:{:b} hex:{:x} octal:{:o}",10,10,20);

//placeholder for debug traits

println!("{:?}",(12,true,"hello"));

println!("10 +10 ={}",10+10);
}

