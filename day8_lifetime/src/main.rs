// fn main() {
//     println!("Hello, world!");
//     // println!("{}",longer("Bonjour!","Hello!"));
//     let x = "bonjour!".to_string();
    
//     let result;
//     {
//         let y = "hello!".to_string();
//         let y1 = y.as_str();
//         result = longer(x.as_str(),y1);
//     }
//     println!("{}",result);

// }
fn longer<'a>(x:  &str, y: &str) -> &'a str{
    let x1 = x.to_string();
    let y1 = y.to_string();
    if x1.len()>y1.len(){
        &x1[0..x1.len()]
    } else {
        &y1[0..y1.len()]
    }
}
fn main() {
    // // let mut x = "bonjour!".chars().collect::<Vec<_>>();
    // // let last = x.len()-1;
    // let mut x:String = "Bonjour!".to_string();
    // x.replace_range(x.len()-1..x.len(),"?");
    // // let result: String = x.into_iter().collect();
    // println!("{}", x);
    let result;
    let a = "something!";
    {
        let b = "somethin!".to_owned();
        let b1 = &b[0..b.len()];
        result = longer(a, b1);
    }
    println!("{}",result);
}
