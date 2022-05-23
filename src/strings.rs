pub fn run(){

   let mut hello:String =String::from("hello");

   println!("{}",hello);
   hello.push('w');
   hello.push_str(" orld!");
   println!("capacity:{}",hello.capacity());
   println!("{}",hello.len());
   println!("{}",hello.contains("orld"));
   println!("{}",hello);
  //
  let mut s =String::with_capacity(10);
  s.push('a');
  s.push('b');

  println!("{}",s);

  //Assertion testing
  assert_eq!(2,s.len());

   for word in hello.split_whitespace(){
       println!("{}",word);
       }

}