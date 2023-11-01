use std::io;

struct User {
    active: bool,
    name: String,
    phonenumber: u32,
    sign_in_count: u32,
}

fn main(){
    let mut name = String::new();

    println!("Enter Name: ");
    io::stdin()
    .read_line(&mut name)
    .expect("failed to read line");

    println!("Enter PhoneNumber: ");
    let mut phonenumber = String::new();

    io::stdin()
    .read_line(&mut phonenumber)
    .expect("failed to take data");

    let phonenumber: u32 = phonenumber.trim().parse().expect("Enter PhoneNumber");

    let mut user1 = build_user(name, phonenumber);
    println!("");
    println!(" details are {},{},{}",user1.name, user1.active, user1.sign_in_count);

}

fn build_user(name: String,  phonenumber: u32) -> User {
    User {
        active: true,
        name,
        phonenumber,
        sign_in_count: 1,
    }
}