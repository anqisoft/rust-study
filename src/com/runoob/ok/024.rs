// 024.rs
// https://www.runoob.com/rust/rust-slice.html

fn main() {
    let s = String::from("broadcast");

    let part1 = &s[0..5];
    let part2 = &s[5..9];

    println!("{}={}+{}", s, part1, part2);
}

/* result:
broadcast=broad+cast
*/
