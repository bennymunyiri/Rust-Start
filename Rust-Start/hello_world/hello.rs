fn main() {
    let target = "hello world!";
    let mut prefix = String::new();
    let mut suffix = target.to_string();

    for c in target.chars() {
        println!("{}", prefix);
        for _ in 'a'..=c {
            prefix.push('.');
        }
        suffix = suffix.chars().skip(1).collect();
        prefix.push(c);
    }
    println!("{}", prefix);
}
