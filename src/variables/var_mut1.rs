fn main()
{
    println!("In example module var_mut.rs we see that immutable variable once assigned with value cannot be reassigned");
    println!("In order to reassign to variable in same scope we can make the variable mutable");
    println!("Example for mutable variables");
    println!("Lets instantiate variable x with keyword mut before variable name and assign it to 2");
    let mut x = 2;
    println!("The value of x: {}",x);
    println!("Now lets reassign the same variable to 1");
    x = 1;
    println!("The value of x: {}", x);
    println!("Unlike previous module this will work fine and program will print 1 because variable x now holds 1 not 2");
    println!("In this way we can make a variable mutable");
}
