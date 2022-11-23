// Vectors - are resizable arrays

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // reassign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single value
    println!("First Value: {}", numbers[0]);

    // get vector length
    println!("Vector length: {}", numbers.len());

    // vectors are stack allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    numbers.pop();

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loopp through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate
    for x in numbers.iter_mut() {
        *x *= 2
    }
    println!("numbers vec: {:?}", numbers);
}
