pub fn example(){
    let s1 = String::from("Hello, World!");
    let s2 = s1;

    println!("{}, world!", s2); // Will not compile with s1 in place of s2
}