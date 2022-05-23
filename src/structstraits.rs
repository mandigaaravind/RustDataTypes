#[derive(Debug)]
pub struct Student{
    name:String,
    rollno:u8,
    marks:u8,
    fave_color:Color
}

#[derive(Debug)]
pub enum Color{
  Red(String),
  Green,
  Yellow
}

impl Student{
   fn print_student(&self)->String{
    //    println!("{}",self.name)
    format!("name= {},roll no is {},marks are {}, favourite color is ",self.name,self.rollno,self.marks)
   }
}

trait Animal{
    fn can_fly(&self)->bool;
    fn is_animal(&self)->bool{
        true
    }
}

impl Animal for Student{
   fn can_fly(&self)->bool{
       false
   }
}

pub fn run(){
    let mut c:Student=Student{name:"Hello".to_string(),rollno:20,marks:30,fave_color:Color::Red("turnAlot".to_string())};
    let  r=Color::Red("Hodown".to_string());
   match r{
       Color::Red(s)=>println!("It is Down to {}",s),
       Color::Green=>println!("it is green"),
       Color::Yellow=>println!("it is yellow"),
   }

   let h:Color=Color::Red("Red it is".to_string());
    c.name="aninymous".to_string(); 
    println!("the name is {}, roll no is {}, marks are {}",c.name,c.rollno,c.marks);
    println!("{}",c.print_student());
    println!("{:?}",h);
    println!("{:?}",c);
    println!("{} {}",c.can_fly(),c.is_animal());
}  