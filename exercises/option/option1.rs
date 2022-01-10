// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// I AM NOT DONE

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    let num1 = Some(13);
    let num2 = Some(99);
    print_number(num1);
    print_number(num2);

    let mut numbers: [Option<u16>; 5] = [None; 5];
    for iter in 0..5 {
        let number_to_add: u16 = {
            ((iter * 1235) + 2) / (4 * 16)
        };

        numbers[iter as usize] = Some(number_to_add);
    }
}
