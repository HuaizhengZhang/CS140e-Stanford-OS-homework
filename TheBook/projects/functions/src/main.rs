fn main() {
    println!("Hello, world!");
    another_function(10, 9);

    new_scope();

    let x = five();
    println!("The value of five function is {}", x);

    let x = plus_one(5);
    println!("The value of plus one is: {}", x)
}


fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn new_scope() {
    let x = 5;

    let u = {
        let x = 3;
        x + 1
    };

    println!("The value of new scope: {}", u)
}


fn five() -> i32 {

    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
