use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("入力の読み込みに失敗しました");
    
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("数値の変換に失敗しました"))
        .collect();
    
    let a = numbers[0];
    let b = numbers[1];
    let c = numbers[2];
    
    if a == b || b == c || c == a {
        println!("Yes");
    } else {
        println!("No");
    }
}
