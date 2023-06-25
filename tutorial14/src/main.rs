fn main() {
    let arr_2: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx: i32 = 0;

    // LOOP

    /* loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("Val: {}", arr_2[loop_idx]);
        loop_idx += 1;
    } */

    // WHILE

    /* while loop_idx < arr_2.len() {
        println!("Val: {}", arr_2[loop_idx]);
        loop_idx += 1;
    } */

    // For

    for val in arr_2.iter() {
        println!("Val: {}", val);
    }
}
