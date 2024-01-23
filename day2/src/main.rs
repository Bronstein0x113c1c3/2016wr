#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn double(p: &mut Point, multiplier: f64) {
    p.x *= multiplier;
    p.y *= multiplier;
}
fn main() {
    // let mut p1 = Point { x: 3.5, y: 7.5 };
    // println!("before: {:?}", p1);
    // double(&mut p1, 2.0);
    // println!("after: {:?}", p1);
    let mut a = vec![0,1];
    for i in 2..30{
        if (i%2==0){
            a.push(a[i32::from(i)>>1]);
        } else{
            a.push(a[i32::from(i)>>1]+1);
        }
    }
    println!("{}",a);
}
