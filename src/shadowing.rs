pub fn example() {
    let x = "Hello, World!";

    {
        let x = x.len();
        print!("A \"Hello, World!\" program generally outputs a string that is {} characters long, ", x);
    }

    print!("with the following format: {}", x);
}