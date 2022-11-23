// Arrays - fixed list where elements are the same data types

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // reassign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single value
    println!("First Value: {}", numbers[0]);

    // get array length
    println!("Array length: {}", numbers.len());

    // arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
