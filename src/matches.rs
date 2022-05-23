pub fn run(){
     let i=5;
     match i{
         0=>println!("0"),
         1 | 2=>println!("1,2"),
         3..=4=>println!("3,4"),
         _=>println!("default")
     }
}