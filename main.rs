
 use std::io;
 fn main() {
      
    println!("Project Calculator");
    
     let mut value1=String::new();
     io::stdin().read_line(&mut value1);
     let value1 : f32   = value1.trim().parse().unwrap();

     let mut value2 =String::new();
     io::stdin().read_line(&mut value2);
     let value2 : f32   = value2.trim().parse().unwrap();
    
     let sum : f32   = value1 + value2;

     let subtract :f32  = value1 - value2;

     let product :f32   = value1 * value2;

     let quotient : f32  = value1 / value2;
    

     println!("The sum of {} and {} is {}. ",value1,value2,sum);
     println!("\n"); 
     println!("The difference between {} and {} is {}. ",value1,value2,subtract);
    println!("\n"); 
     println!("The product of {} and {} is {}. ",value1,value2,product);
     println!("\n"); 
     println!("The quotient of {} and {} is {}. ",value1,value2,quotient); 
      
     println!("\n");
     println!("End");
 }

