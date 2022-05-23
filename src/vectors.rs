// use std::mem;



// pub fn two_sum(nums:  Vec<i32>, target: i32) -> Vec<i32> {
//     let mut nano:Vec<i32>=vec![];
//     nano.push(6);
//     nano.push(7);
//     nano
// }

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut numbers:Vec<i32>=vec![];
    let c:usize=nums.len();
    for i in 0..c-1{
        for j in i+1..c{
            if nums[i]+nums[j]==target{
                numbers.push(i as i32);
                numbers.push(j as i32);
                break;
            }
        }
    }
    numbers
}


pub fn summ(){
    let mut numbers:Vec<i32>=vec![1,2,3,4,5];
    let  nano:Vec<i32>=two_sum( numbers,9);
    println!("{:?}",nano);
}

pub fn run(){    
    let mut &numbers:Vec<i32>=vec![1,2,3,4,5];
   
   let nano:Vec<i32>=two_sum( numbers,9);
   println!("{:?}",nano);
    //re-assign value
     summ();
    //add on to number
    numbers.push(5);
   numbers[2]=20;

    println!("{}",numbers.len());

    println!("{:?}",numbers);
    //pop off the last value
    numbers.pop();
    println!("{:?}",numbers);
    println!("{}",numbers.len());

    let slice:&[i32]=&numbers[1..3];
    println!("{:?}",slice);

    //loop 
    for x in numbers.iter(){
        println!("{}",x);
    }

    println!("{:?}",numbers);
    //loop mutate values
    for x in numbers.iter_mut(){
        *x *=2;
    }
    println!("{:?}",numbers);

}