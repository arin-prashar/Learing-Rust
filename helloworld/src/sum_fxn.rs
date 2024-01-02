fn main() {
    let p:i32=10;
    let q:i32=20;
    let res=sum(p,q);
    println!("Sum of {} and {} is {}",p,q,res);
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}