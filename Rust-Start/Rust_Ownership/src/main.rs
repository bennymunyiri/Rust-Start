lfn main(){
    let x = String::from("Hello");

    takes_ownership(x);

    let y = 5;

    make_copy(y);
    println!("number is {y}")

}

fn takes_ownership(x: String){

    println!("{x}, world");
}

fn make_copy(y: i32){
    println!("{y} the number");
}