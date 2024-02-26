fn main() {
    // variables mutability - by defaulkt they are immutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x= 6;
    println!("The value of x is: {}", x);

    // constant variables in rust
    const YEAR_BORN: i32 = 1998;
    println!("I was born in: {}", YEAR_BORN);
}
