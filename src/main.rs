use std::io;

fn main() {
    println!("the is_even of {}",is_even(7));

    
   
   // let divide=results::divisor(4,0); 
   // println!("{:?}",divide);
 //  structstraits::run();
//    let divide1:Option<i32>=options::divide(4,2);
//    let divide2:Option<i32>=options::divide(5,3); 
//    println!("{:?} and the unwrapped version is {}",divide1,divide1.unwrap());
//    println!("{:?} and the unwarpped version is {}",divide2,divide2.unwrap());

   
//   match divide{
//       Ok(v)=>println!("{}",v),
//       Err(v)=>println!("{:?}",v)
//   }

//let _res=divide.expect("we crashed");
//println!("{}",res);

//println!("{}",divide.unwrap_or(200));

  
}

pub fn is_even(num:u8)->bool{
    let digit:u8=num%2;
    digit==0
}

