fn main() {
    let mut a = vec![1,23,4,56,7,4,2,8];
    print(&a);
    a.push(890);
    print(&a);
}
fn print(list: &Vec<i32>){
    for i in list{
        print!("{} ",i);
    }
    println!();
}