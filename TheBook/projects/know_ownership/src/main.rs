fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    move_s1_s2();

    clone_s1_s2();


    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

}


fn move_s1_s2() {

    let s1 = String::from("hello");

    let s2 = s1;

    println!("{}, world!", s2)
}


fn clone_s1_s2() {

    let s1 = String::from("hello");

    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

}


fn gives_ownership() -> String{

    let some_string = String::from("hello");

    some_string

}

fn takes_and_gives_back(a_string: String) -> String {

    a_string
}