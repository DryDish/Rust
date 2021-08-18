pub fn array_test() {
    let array_thing = [1, 2, 3, 4, 5];
    let other_array: [u8; 5] = [1, 125, 254, 255, 246];

    let slice_other = &array_thing[1..3];
    println!(
        "array thing: {:?}, other array: {:?}, slice of other: {:?}",
        array_thing, other_array, slice_other
    );
    print!("The array from a loop! ");
    let mut counter = 0;
    loop {
        print!("{}", array_thing[counter]);
        counter += 1;
        if counter >= 5 {
            break;
        }
    }
    println!();
}

pub fn vector_test() {
    /*
    Alternative to vec!
    Type can be inferred from the first push into it by compiler, or explicitly stated
    let mut people: Vec<&str> = Vec::new();
    people.push("Pineapple");
     */

    // Type is known by what is added, but can be explicitly stated: Vec<&str>
    let mut people = vec!["Peter", "Alice", "Bob", "Tom"];
    people.push("Thomas");

    println!("Vector: {:?}", people);

    let mut capacity_test: Vec<i8> = Vec::new();
    println!("Capacity: {}", capacity_test.capacity());

    capacity_test.push(1);
    println!("Capacity: {}", capacity_test.capacity());

    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);

    println!("Capacity: {}", capacity_test.capacity());

    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);

    println!("Capacity: {}", capacity_test.capacity());
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);
    capacity_test.push(1);

    println!("Capacity: {}", capacity_test.capacity());

    // Declaring capacity also determines what the doubling value is
    let mut capacity_vector: Vec<u8> = Vec::with_capacity(12);

    println!("Capacity: {}", capacity_vector.capacity());
    for _ in 0..13 {
        capacity_vector.push(2);
    }
    println!("Capacity: {}", capacity_vector.capacity());
    for _ in 0..13 {
        capacity_vector.push(2);
    }
    println!("Capacity: {}", capacity_vector.capacity());
}

pub fn tuple_test() {
    let simple_arr = [1, 2, 3];
    let rand_tuple = ("this", 1, [4, 5, 6]);
    println!("{:?}", rand_tuple);
    let (a, b, c) = (simple_arr[0], simple_arr[1], simple_arr[2]);
    println!("first: {}, second: {}, third: {}", a, b, c);
}