// TODO: Add the missing type of the argument `num` after the colon `:`.
fn call_me(num:i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
    call_you(12);
}
fn call_you(num:i32){
    for x in  0..num{
        println!("papitas {}", x-1)
    }
}
 
