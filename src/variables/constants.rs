fn main()
{
    println!("This example displays difference between constants and immutable variables");
    println!("Constants defined with keyword const and immutable variables defined with keyword let");
    println!("Constants needs to be type annotated");
    const MYCONST: i32 = 10;
    println!("The value of MYCONST: {}", MYCONST);
    let x = 10;
    println!("The value of x: {}", x);
}
