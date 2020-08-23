fn main()
{
    println!("In this module we will explore shadow for mutable variables");
    let mut x = 1;
    println!("The value of x:{}", x);
    let mut x = x+1;
    println!("The value of x:{}", x);
    x = x*2;
    println!("The value of x:{}", x);
}
