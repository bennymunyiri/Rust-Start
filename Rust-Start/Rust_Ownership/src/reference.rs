fn main(){
    let mut s = String::from("Hello");

    change(&mut s);
    println!("{s}");
}

fn change(some_string: &mut String){
    some_string.push_str(", Benson");
}

// fn size() {
//     let size_of_usize: usize = std::mem::size_of::<usize>();
//     println!("Size of usize in bytes: {}", size_of_usize);
// }
