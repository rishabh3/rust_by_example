fn main()
{
    println!("This module lays out an example for variable shadow");
    println!("Using the keyword let we can redefine the same variable in same scope");
    println!("Lets see the example");
    println!("Lets assign variable x with 2");
    let x = 2;
    println!("The value of x: {}", x);
    println!("Lets redefine variable x with x+1");
    let x = x+1;
    println!("Value of x: {}", x);
    println!("Lets redefine variable x with x*4");
    let x = x*4;
    println!("Value of x: {}", x);
    println!("This is known as shadowing as last x overshadows the previous x and that is the reason final x value is 12 and not just 3");
}
