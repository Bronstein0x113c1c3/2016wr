use std::rc::Rc;

#[derive(Debug)]
struct Student{
  name: Rc<String>,
  id: i32,  
}

impl Student{
    fn info(&self) -> String{
        format!("Name: {} \n ID: {}",self.name,self.id) 
    }
    fn get_name(&self) -> String{
        (*self.name).clone()
    }

}

fn main() {
    let mut list = vec![];
    for i in 0..503{
        let new_student = Student{
            name: Rc::new(format!("Bonjour!{}",i.to_string())),
            id: i,
        };
        list.push(new_student); 
    }
    for i in list.iter(){
        println!("{}",i.info());
        println!("{}",i.get_name());
    }
}
