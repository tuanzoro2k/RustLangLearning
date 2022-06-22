#[derive(Debug)]
struct Fibonacci {
    a: u32,
    b: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    // Trả về số fibonaci tiếp theo dựa trên kiểu dữ liệu struct Fibonacci
    fn next(&mut self) -> Option<u32> {
        let tmp = self.b;
        self.b = self.a;
        self.a += tmp;
        Some(self.b)
    }
}

// Khởi tạo ban đầu cho Fibonaci: 0, 1
fn fibonacci_numbers() -> Fibonacci {
    Fibonacci { a: 1, b: 0 }
}

fn main() {
    //     Vì struct Fibonacci có implement trait Iterator của Rust nên
    // có thể dùng câu lệnh for dc
    // Câu lệnh for bản chất sẽ chuyển qua trait Iterator nên instance của
    // struct Fibonacci có thể duyệt được,
    // Mỗi lần duyệt sẽ tự động chạy function signature next() trên
    // Nên cần implement hàm next() cho struct Fiboncci.
    for number in fibonacci_numbers() {
        println!("{}", number);
    }
}
