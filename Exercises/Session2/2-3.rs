fn main() {
    let mut values = vec![10, 11, 12];
    let v = &mut values;

    let mut max = 0;

    for n in v.iter() {
        max = std::cmp::max(max, *n);
    }

    println!("max is {}", max);
    println!("Converting to percentages of maximum value...");
    for n in v {
        *n = 100 * (*n) / max;
    }
    println!("values: {:#?}", values);
}
