
fn main() {
    let mut content = String::new();
    let val: i64 = 20;
    defint(&mut content, val);
    println!("test string {}", content);
}

fn defint(content: &mut String, val: i64) -> () {
    let x = format!("mov {}, %eax", val);
    content.push_str(&x);
}
