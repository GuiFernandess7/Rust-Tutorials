fn main() {
    println!("Hello, world!");
    // Data Types (Scalar Types):
    // Integers:
    // -    i8
    // -    i16
    // -    i32
    // -    i64
    // -    i128
    // -    u32 (can't be negative)
    let _x: i32 = 2; // 32 bits for number 2
    let _floating_point: f32 = 10.9;
    let _true_or_false: bool = true; // can use 0 also
    let _letter: char = 'a';

    // Compount Types:
    let mut my_tuple: (i32, bool, char) = (1, true, 's');
    my_tuple.0 = 10; // mutate a tuple first value
    println!("{}", my_tuple.0);

    // Pattern matching
    let (a, b, c) = my_tuple;
    println!("{}, {}, {}", a, b, c);

    // Arrays:
    let mut my_array: [i32; 5] = [1, 2, 3, 4, 5];
    my_array[4] = 3;
    println!("{}", my_array[4]);

    // Slicing:
    let slice_numbers = [24, 42, 45, 53, 69];
    println!("{:?}", &slice_numbers[1..3]);
}
