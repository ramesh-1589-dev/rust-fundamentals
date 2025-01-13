fn main() {
    println!("Hello, Welcome to rust programming language..!");

    //Comments

    //Variables
    //i: signed integer:  + or -
    //u: Unsigned integer: no sign only positive
    let i: i32   =10; 
    println!("The value of variable i is : {i}");

    let mut y: i32 =5;
    println!("The value of mutable integer before changed is {y}");
    y=10;
    println!("The value of mutable integer after changed is {y}");

    // Numbers: integer, float, boolean: Sored in Stack Info.

    let f: f64 = 15.44 ;
    println!("The value of float variable is {f}");

    let is_active: bool= true;
    
    println!("The value of boolean variable is {is_active}");

    //Operations

    let add = i + 10  + 10;

    println!("The Addition of two numbers is {add}")

    //Char: In single quotes
    // let n: char ='R';
    // println!("Hi my name is {}", n);

}
