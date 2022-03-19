fn main() {

    // Variables
    let number: i8 = 127;
    println!("Number is {} and this length in bytes is {}", number, std::mem::size_of_val(&number));

    let decimal: f64 = 32.1000005;
    println!("Float number is {} and this length in bytes is {}", decimal, std::mem::size_of_val(&decimal));

    let boolean: bool = false;
    println!("Bool is {} and this length in bytes is {}", boolean, std::mem::size_of_val(&boolean));

    let char: char = 'É';
    println!("Char is {} and this length in bytes is {}", char, std::mem::size_of_val(&char));

    // Constants

    const PI: f32 = 3.14;
    println!("Constant PI is {}", PI);

    static mut _VARIABLE: i64 = 300;
    unsafe {
        println!("Static variable is {}", _VARIABLE);
    }

    // Functions

    fn sum(n1: i32, n2: i32) -> i32 {
        return n1 + n2;
    }

    println!("A soma de 1 + 5 = {}", sum(1, 5));

    // Match Statments

    let language: &str = "Rust";
    let purpose: &str = match language {
        "Rust" => "Low Level",
        _ => "Undefined"
    };

    println!("The language is {} and this purpose is {}", language, purpose);

    // Ownership

    let string = String::from("Heap Allocation");
    steal(&string); 

    fn steal(string: &String) {
        println!("{}", string);
    }

}