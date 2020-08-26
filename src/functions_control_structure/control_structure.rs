fn main()
{
    println!("Example of simple if else statement");
    let mut n = 10;
    if n < 100 {
        println!("n is less than 100");
    }
    else if n < 20 {
        println!("n is less than 20");
    }
    else {
        println!("n is greater than 100");
    }
    println!("Loop example");
    loop {
        println!("This is infinite loop until break is called");
        break;
    }
    while n < 20 {
        println!("n is less than 20: {}" , n);
        n += 1;
    }
    let x = [1,2,3,4,6,7];
    for elem in x.iter() {
        println!("elem: {}", elem);
    }
}
