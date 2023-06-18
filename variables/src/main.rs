const MAX_POINT: u32 = 5;

fn main() {
    let mut x = 5;
    println!("The value x : {}", x);
    x = 6;
    println!("The value x : {}", x);

    println!("The MAX_POINT : {}", MAX_POINT);

    let a = 3;
    let a = a * 3;
    let a = a * 4;

    println!("The value a : {}", a);

    let _space = "   "; // mut 붙이면 에러 
    let _space = _space.len();
} //Shadowing
