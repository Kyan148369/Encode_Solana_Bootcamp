fn main() {
    print!("Hello, world! \n");
    fizzbuzz();

}

fn fizzbuzz() {
    let mut j: u32 = 0;
    for i in 1..302{
        if i%3 ==0 && i%5==0{
            println!("fizz buzz\n");
            j += 1;
            println!("No of time fizz buzz occured is {}\n",j );

    }
        else if i%3 == 0 {
            print!("fizz\n")
        }
        else if i%5 ==0 {
            print!("buzz\n");
        }
        
    } 
}