use std::rc::Rc;

// concept of Const list of i32
// This wont compile
#[derive(Debug)]
pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

impl List {
    pub fn show(&self) -> String {
        match self.to_owned() {
            List::Cons(current, next) => format!("{} {}", current, next.show()),
            List::Nil => return "EOF".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::{ops::Deref, rc::Rc};

    use super::List::{Cons, Nil};

    #[test]
    fn test_box() {
        let cajita = Box::new(5);
        println!("b = {}", cajita);
    }

    #[test]
    fn test_cons_list() {
        let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));
        list.show();
    }

    #[test]
    fn test_deref_normal() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_deref_using_box() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[test]
    fn test_deref_using_my_box() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_deref_coersion_with_my_box() {
        let m = MyBox::new(String::from("Rust"));

        fn hello(name: &str) {
            println!("Hello, {name}!")
        }

        hello(&m);
    }

    #[test]
    fn test_reference_counting_using_list() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let b = Cons(3, Rc::clone(&a));
        let c = Cons(4, Rc::clone(&a));

        b.show();
        c.show();
    }
}
