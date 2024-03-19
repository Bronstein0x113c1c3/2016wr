fn main() {
    println!("Hello, world!");
    //ep1: array...
    let x = ["English", "This", "sentence", "a", "in", "is"];
    print!("{} {} {} {} {} {} \n", x[1], x[5], x[3], x[2], x[4], x[0]);
    //nah, it's just an immutable array
    for x in x.iter(){
        println!("{}",x);
    }
    //ep2: 
    let mut x = ["This", "is", "a", "sentence"];
    x[2] = "a nice";
    print!("{} {} {} {}.", x[0], x[1], x[2], x[3]);
}
