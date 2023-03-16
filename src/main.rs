#[allow(non_snake_case)]

fn f(n: u32) -> u32{
    if n==0{1}
    else if n==1{1}
    else {f(n-1)+f(n-2)}
}

fn main() {
    for i in 0..10{
        println!("{}", f(i));
    }
}
