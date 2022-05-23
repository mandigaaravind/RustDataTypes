#[derive(Debug)] 
enum Movement{
    Up,
    Down,
    Left,
    Right1
}

#[derive(Debug)] 
enum Movement2{
    A,
    B(i32),
    C{x:i32,y:i32}, 
}

fn move_avatar(m: Movement){
    match m{
        Movement::Up=>println!("Avatar moving up"),
        Movement::Down=>println!("Avatar moving down"),
        Movement::Left=>println!("Avatar moving left"),
        Movement::Right1=>println!("Avatar moving right")
    }
}
pub fn run(){
    let avtar1=Movement::Left;
    let avtar2=Movement::Right1;
    let avtar3=Movement::Up;
    let avtar4=Movement::Down;
    let b=Movement2::B(6);
    let c=Movement2::C{x:10,y:20};
    println!("{:?}",b);    
    move_avatar(avtar2);

    if let Movement2::B(val)=b {
        println!("{}",val);
    }

    if let Movement2::C{x,y}=c {
       println!("{} {}",x,y);
    }
}