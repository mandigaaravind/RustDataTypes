pub fn run(){    
    let mut numbers:[i32;5]=[1,2,3,4,5];

    //re-assign value
    numbers[2]=20;
    println!("{:?}",numbers);
    println!("{}",numbers.len());

    let slice:&[i32]=&numbers[0..2];
    println!("{:?}",slice);

    for i in numbers.iter_mut(){
       *i=*i*2;
    }
    println!("{:?}",numbers)

}  