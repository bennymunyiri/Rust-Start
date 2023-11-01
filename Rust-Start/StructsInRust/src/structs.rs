struct Address{
    name: String,
    phonenumber: String,
    email: String,
    active: bool,
}

fn main (){
    let mut user1 = Address {
        name: String::from("Benson"),
        phonenumber: String::from("254797096114"),
        email:String::from("munyiribenson@gmail.com"),
        active: true,
    };

    let mut user2 = Address {
        phonenumber: String::from("25496123491"),
        ..user1
    };

    user1.name = String::from("Munyiri");
    println!("{}, {}, {}", user1.name, user2.phonenumber, user2.name);
}
