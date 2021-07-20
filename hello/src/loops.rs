// Loops are loops
pub fn run() {
    let mut count = 0;
    let mut counts = 0;
    // infinite loop
    loop {
        counts += 1;
        println!("Number: {}", counts);

        // Exit condition
        if counts == 20 {
            break;
        }
    }

    // While loop (FizzBuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", count);
        }
        count += 1;
    }

    println!("---------------------------------------------------");

    // For range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", x);
        }
    }
}
