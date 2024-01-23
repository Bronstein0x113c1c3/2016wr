fn main(){
    let mut res = 0;
    for i in (0..1<<4){
        if (i&(1<<7)>0)||(i&3==0){
            res+=1;
        }
    }
    println!("{}",res);
}