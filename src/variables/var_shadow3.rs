fn main()
{
    println!("This module lays down example of difference between mutable and shadow");
    println!("Lets define variable x with value 2");
    let mut x = 2;
    println!("The value of x: {}",x);
    println!("Lets try to assign a different type value to x");
    x = "Hello";
    println!("The value of x: {}", x);
}
