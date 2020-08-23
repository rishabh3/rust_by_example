fn main()
{
    println!("With variable shadow we can change types of variables as we are redefining the variables");
    let x = 1;
    println!("The value of x: {}", x);
    println!("Lets redefine variable x");
    let x = "Hello";
    println!("The value of x:{}", x);
}
