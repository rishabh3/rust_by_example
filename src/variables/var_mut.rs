fn main()
{
    println!("This module gives example of immutable variables");
    println!("Defining variable x and value it holds is 2");
    let x = 2;
    println!("The value of x: {}", x);
    println!("Now try to change the value of the variable to 1");
    x = 1;
    println!("This is a compile time error because immutable variables once assigned with values will never change");
}
