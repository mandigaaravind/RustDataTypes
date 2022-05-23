pub fn run(){ 
  

    let mut count=0;
    // loop {
        
    //   //  println!("{}",count);

    // if count==100{
    //     break;
    // }
    
    
   while count<=100{
      if count%15 == 0{
           println!("Fizzbuzz");
      }
      else if count%3==0{
          println!("FIzz");
      }
      else if count%5==0{
          println!("BUzz");
      }
      else{
          println!("{}",count)
      }
      count+=1;
   }

   
    }

