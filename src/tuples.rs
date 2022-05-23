pub fn run(){

   let person:(&str,&str,i8)=("brad","mass",37);
   println!("{} is from {} and also {}",person.0,person.1,person.2);

   let (a,b,c)=person;

   println!("first {}, second {}, third {}",a,b,c);

}