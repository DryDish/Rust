pub fn run() {
    greeting("Hello", "Peter");

    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("{}", get_sum);

    // Closure (can use things within scope)
    let n3: i32 = 10;                                   // n3 from scope
    let add_nums = |n1: i32, n2: i32 | n1 + n2 + n3;
    println!("Sum: {}", add_nums(3, 3));

}

fn greeting(greet: &str, name: &str) {
    println!("{} {} nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
