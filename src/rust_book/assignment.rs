pub fn example() {
    copying();
    moving();
    cloning();
}

fn copying() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y); // Copying x to y does not move x to y because the information is stored on the stack.
}

fn moving() {
    let s1 = String::from("Hello");
    let s2 = s1;

    println!("{}, World!", s2); // Will not compile with s1 in place of s2
}

fn cloning() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("{}, {}!", s1, s2); // Clone performs a copy of the heap space.
}