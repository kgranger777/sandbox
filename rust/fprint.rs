fn main() {
    // positional arguments example
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "alice","bob");
    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");
    println!("my name is {0}, {1} {0}", "bond","james");

    // not sure what this does...
    // dead code - lint that warns about unused functions. the below attribute disables the lint.
    #[allow(dead_code)]
    struct Structure(i32);
    // this will not compile because "Structure" does not implement fmt::Display
    // println!("this struct `{}` won't print...", Structure(3));

    // formatting using masking (colon character)
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:.width$}");

    let pi = 3.141592;
    println!("pi is roughly {:.3}", pi);
}