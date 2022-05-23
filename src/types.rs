pub fn run(){

  /*
  Integers:u8,i8,..upto u128,i128
  floats :f32 ,f64
  boolean(bool)
  characters(char)
  tuples
  arrays
  */

    let x=1;
    let y=2.5;
    let z:i64=45456273647;
    println!("max i32:{}",std::i32::MAX);

    let is_active:bool=true;
    let is_greater:bool=10>15;

    let a1='a';
    println!("{:?}",(x,y,z,is_active,is_greater,a1));
}