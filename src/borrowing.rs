fn main() {
   let mut a=String::from("hello");
   let b=&a;
   let c=&a;
   println!("{}",a);
   println!("{} {}",b,c);
   let d= match a.pop(){
      Some(f)=>f,
      None=>'4',
  };
  println!("{}",a);
  println!("{}",d);
  a.pop();
   println!("{}",a);
   let e=&mut a;
   // a.pop(); --doesnt work as e is already in scope.
  //  println!("{}", a);
   println!("{}",e);
   e.pop();
   println!("{}",e);

   own_and_borrow(&mut a,d);
   println!("{}",a);
}

pub fn own_and_borrow(a:&mut String,d:char){
    a.pop();
}