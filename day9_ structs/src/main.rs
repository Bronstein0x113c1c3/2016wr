// use std::{cell::RefCell, rc::Rc, vec};

// #[derive(Debug)]
// struct Class{
//     name: RefCell<String>,
//     list_student : Rc<RefCell<Vec<Student>>>
// }
// #[derive(Debug)]
// struct Student{
//     name: String,

// }
// fn main() {
//     let mut class1 = Class{
//         name: RefCell::new("5A4".to_string()),
//         list_student: Rc::new(
//             vec![].into()
//         ),
//     };
//     class1.list_student.clone().borrow_mut().push(
//     Student { name: "Something".to_string() }
//     );
//     class1.list_student.clone().borrow_mut().get_mut(0).unwrap().name = "Bonjour!".to_string();
//     for x in class1.list_student.borrow().iter(){
//         println!("{:?}",x.name);
//     }
// }

use std::cell::RefCell;
use std::{rc::Rc, vec};
#[derive(Debug)]
struct Class {
    name: RefCell<String>,
    list_student: Rc<RefCell<Vec<Student>>>,
}
#[derive(Debug)]
struct Student {
    name: String,
}
fn main() {
    let mut c1 = Class {
        name: RefCell::new("5A4".to_string()),
        list_student: Rc::new(RefCell::new(vec![])),
    };
    let mut l1 = c1.list_student.clone();
    let mut l2 = c1.list_student.clone();
    l1.borrow_mut().push(Student {
        name: "???".to_string(),
    });
    println!("{:?}", l1.borrow()[0].name);
    println!("{:?}", l1.borrow()[0].name);
}
