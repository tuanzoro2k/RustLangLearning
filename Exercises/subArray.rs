fn main() {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 10, 8];

    let mut i = 0;
    let mut j = 1;
    let mut check = false;
    while i < org_arr.len() {
        if org_arr[i] == sub_arr[0] {
            let mut start = i;
            check = true;
            while j < sub_arr.len() {
                if sub_arr[j] == org_arr[start] {
                    check = false;
                    break;
                }
                start += 1;
                j += 1;
            }
        }
        if check {
            break;
        }
        i += 1;
    }
    if check {
        println!("Subarray");
    } else {
        println!("Not a subarray");
    }
}
