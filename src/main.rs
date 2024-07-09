use rand::Rng;
fn main() {
    hello();
    // let mut  x:u8 = 3; //mutable variable 
    // let y:u8 = 4;
    // x=2;
    // println!("x+y={}",x+y);
    guess_number()
}
// i8 = -(2^7) to -(2^7)-1 
fn hello(){
    println!("hello");
}

fn guess_number(){

    let mut number = String::new();
    std::io::stdin().read_line(&mut number).expect("Fail to read");
    let mut gen_number = rand::thread_rng();
    let x:i8 = number.trim().parse().expect("Invalid input");
    let num1:u8 = gen_number.gen_range(0..10);
    println!("The generated number:{}",num1);
   
    println!("The number {}",number);
    while (x!=gen_number) {
        
    }
}