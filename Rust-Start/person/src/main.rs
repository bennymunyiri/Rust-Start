use std::io;

#[derive(Debug)]

struct Person{
    name: String,
    age: u32,
}

#[derive(Debug)]
enum Gender{
    Man,
    Woman,
}

fn main(){
    println!("Enter your name: ");

    let mut name = String::new();

    io::stdin()
    .read_line(&mut name)
    .expect("Cannot read line");

    name = name.trim().to_string(); 

    println!("Enter Age: ");
    
    let mut age_input = String::new();

    io::stdin()
    .read_line(&mut age_input)
    .expect("Cannot read line");

    let age: u32 = age_input.trim().parse().expect("couldn't convert");

    let person = Person { name, age }; 

    let gender: Gender;
    loop {

        println!("Enter Gender Choice! 1 for man 2 for woman");
        let mut gender_choice = String::new();
        io::stdin()
        .read_line(&mut gender_choice)
        .expect("couldnt read line");


        let gender_choice: u32 = gender_choice.trim().parse().expect("couldn't convert");

        gender = match gender_choice{
        1 => Gender::Man,
        2 => Gender::Woman,
        _ => {
            continue
        }
    };
        break;
    }



    println!("Person Details:");
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("Gender: {:?}", gender);


}