fn main()
{
    println!("4 Scalar and 2 Compound Types");
    println!("Rust is statically typed language so compiler needs to know the type");
    println!("Compiler can infer type by value or types can be annotated in the code");
    println!("Default int type is i32, float type is f64");
    let x: i32 = 100;
    println!("The value of x: {}", x);
    let x: f32 = 2.5;
    println!("The value of x: {}", x);
    let x: bool = true;
    println!("The value of x: {}", x);
    let x: char = 'Z';
    println!("The value of x: {}", x);
    println!("Compound type tuple: fixed length heterogeneous values");
    let x: (i32, f64, bool, char) = (100, 2.5, true, 'Z');
    println!("The value of x: {:?}", x);
    println!("Destructure: unpack the value of tuple to independent variables");
    let (a, b, c, d) = x;
    println!("All the values will be unpacked into new variables a, b, c, d");
    println!("a={}, b={}, c={}, d={}", a, b, c, d);
    println!("Compound type array: fixed length homogeneous values");
    let x: [i32; 5] = [1,2,3,4,5];
    println!("Defines the array of type i32 and with length 5");
    println!("Value of x: {:?}", x);
    println!("Defining array with same values");
    let x: [i32; 5] = [6; 5];
    println!("Value of x: {:?}", x);
}
