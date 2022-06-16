fn main() {
    let x = change_value(10, &mut 20);
}

fn change_value(input: u32, output: &mut u32) -> u32 {
    if input == 1 {
        *output = 3;
    } else {
        *output = 4;
    }

    *output
}
