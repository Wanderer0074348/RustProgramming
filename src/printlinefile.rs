fn main() {
    println!("Hello World");
// println!("Hello World")
// {} acts as a placeholder
    println!("This is a {} programme", "rust");
// does not leave a line when printing
    print!("This is a programme\n");
// can do basic math 
    println!("{}+{}={}",5,5,5+5);
// format!: write formatted text to String
// print!: same as format! but the text is printed to the console (io::stdout).
// println!: same as print! but a newline is appended.
// eprint!: same as print! but the text is printed to the standard error (io::stderr).
// eprintln!: same as eprint! but a newline is appended.

// integers
    let x:i32 = 50;
    println!("{}",x);
// float
    let y:f64 = 24532429258.00;
    println!("{}",y);
// Max value of datatype eg) f64
    println!("{}",std::f64::MAX);
// unsigned integers
    let p:u32=53;
    println!("{}",p);
}
