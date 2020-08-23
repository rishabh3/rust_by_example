fn main()
{
    println!("Indexing and access elements from tuple and array");
    let x = (1, 2.5, 'A');
    let y: [i32;5] = [2;5];
    println!("x.0 = {}, x.1 = {}, x.2 = {}", x.0, x.1, x.2);
    println!("y[0] = {}, y[1] = {}, y[2] = {}, y[3] = {}, y[4] = {}", y[0], y[1], y[2], y[3], y[4]);
}
