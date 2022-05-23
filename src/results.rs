
#[derive(Debug)] 
pub enum Myerror{
    Error1
}

#[derive(Debug)]
pub enum Res<T,E>{
   Thing(T),
   Error(E),
}


pub fn divisor(a:i32,b:i32)->Res<i32,String>{
    if b==0{
      return Res::Error("Cannot divide by 0".to_string())
    }
     Res::Thing(a/b)
}

pub fn Xenotype(){
    let c=divisor(20,7);
    println!("{:?}",c);

    if let Res::Thing(v)=c{
        println!("{}",v+100);
    }
    match c{
       Res::Thing(s)=>println!("has divisor of {}",s),
       Res::Error(s)=>println!("not divisible by zero {}",s)
    }
}

//Err, an enum that contains an error code
//Ok(value), A wrapper that contains a value
pub fn divide(dividend:i32,divisor:i32)-> Result<i32,Myerror>{
    if dividend % divisor !=0 {
        Err(Myerror::Error1)
    }
    else{
        Ok(dividend/divisor)
    } 
}