// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    num * num
}
//This is unnecesary, the return part;
fn apple(num : i32) -> i32{
    return num * 1231231
}

fn main() {
    let ans = apple(3);
    println!("The answer is a {ans}");
    let answer = square(3);
    println!("The square of 3 is {answer}");
}


