fn main()
{
    println!("Example for shadow const");
    const X: i32 = 10;
    println!("Value of X: {}", X);
    println!("Redefine const X with value 100");
    const X: i32 = 100;
    println!("Value of X: {}", X);
}
