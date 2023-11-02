use std::io;
fn main(){
    let a = [1,2,355,3,5];

    println!("Enter the index");

    let mut index =  String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Unable to read data");

    let index: usize = index.trim().parse().expect("Not applicable");

    let element = a[index];

    println!("This is the location of index {element}");


}