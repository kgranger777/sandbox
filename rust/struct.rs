#[derive(Debug)]
struct IntPrint(i32);

#[derive(Debug)]
struct PrintIntPrint(IntPrint);

fn main() {
    let intvar = 3;
    println!("{:?}",IntPrint(intvar));
    println!("{:?}",PrintIntPrint(IntPrint(intvar)));
}