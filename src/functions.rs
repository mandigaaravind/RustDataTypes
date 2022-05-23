pub fn run(){
    greeting("how are you","Aru");

    let k= add(3,4);
    println!("{}",k);
}

fn greeting(greet:&str, name:&str)
      {
          println!("{} {},nice to",greet,name)
      }

fn add(mut n1:i32,mut n2 :i32)->i32
{
    n1+=2;
    n2+=3;
    n1+n2
}