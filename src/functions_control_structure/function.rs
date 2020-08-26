fn another_function()
{
    println!("This is another function without any parameter");
    println!("This function returns nothing");
}

fn greet(name: &str)
{
    println!("Hello, {}. Good Morning!", name);
}

fn add(a: i32, b: i32) -> i32
{
    return a+b;
}

fn main()
{
    another_function();
    greet("Peter");
    let z = add(1,2);
    println!("z = {}", z);
    let x = {
        println!("This is just like anonymous functions");
        let x = 10;
    x+1
    };
    println!("The value of x: {}", x);
}
