pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use std::{borrow::Borrow, fmt::Display};

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_deref() {
        use std::ops::Deref;

        struct MyResult<T, E> {
            inner: Result<T, E>,
        }

        impl<T, E> Deref for MyResult<T, E> {
            type Target = Result<T, E>;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl<T, E> From<Result<T, E>> for MyResult<T, E> {
            fn from(inner: Result<T, E>) -> Self {
                Self { inner }
            }
        }
    }

    /// .
    #[test]
    fn test_a() {
        println!("{:?}", &8_u32.to_le_bytes());

        let a = {
            let mut a = 1;
            a += 1;
            if a > 0 {
                return;
            }
            a
        };

        println!("{}", a);
    }

    #[test]
    fn test_lifetime() {
        let r;
        {
            let x = 5;
            r = &x;
            println!("r: {}", r);
        }
        //println!("r: {}", r);
    }

    #[test]
    fn test_fn_lifetime() {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let x = "abcd";
        let y = "abc";
        println!("{}", longest(x, y));

        let string1 = String::from("abcd");
        let result;

        {
            let string2 = String::from("xyz");
            result = longest(&string1, &string2);
            println!("The longest string is {}", result);
        }
        //println!("The longest string is {}", result);
    }

    #[test]
    fn test_method_lifetime() {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        impl<'a> ImportantExcerpt<'a> {
            fn level(&self) -> i32 {
                3
            }

            fn announce_and_return_part<'b>(&self, announcement: &'b str) -> &'b str
            where
                'a: 'b,
            {
                println!("Attention please: {}", announcement);
                self.part
            }
        }
    }

    #[test]
    fn test_static_lifetime() {
        fn print_author(author: &'static str) {
            println!("{}", author);
        }

        {
            let x = "abcd";
            print_author(x);
        }

        fn static_bound<T: std::fmt::Display + 'static>(t: T) {
            println!("{}", t);
        }

        fn static_bound_ref<T: std::fmt::Display + 'static>(t: &T) {
            println!("{}", t);
        }

        {
            let x = "abcd";
            static_bound(x);
            let i = 32;
            static_bound(i);
            //static_bound(&i);
            static_bound_ref(&i);
        }
    }

    #[test]
    fn test_display() {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Serialize, Deserialize)]
        struct MyTest {
            id: u32,
            name: String,
        }

        impl Display for MyTest {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                //write!(f, "id: {}, name: {}", self.id, self.name)
                //f.write_fmt(format_args!("id: {}, name: {}", self.id, self.name))
                f.write_str(&serde_json::to_string(self).unwrap())
            }
        }

        let t = MyTest {
            id: 1,
            name: "abcd".to_string(),
        };

        println!("{t}");
        println!("{t:?}");
    }

    #[test]
    fn test_struct_borrow() {
        #[derive(Debug, Clone)]
        struct Test {
            name: String,
            age: i32,
        }

        fn aa(src: String) {
            println!("{}", src)
        }

        let t = Test {
            name: "abcd".to_string(),
            age: 10,
        };

        aa(t.name);

        //let t = t.clone();

        //println!("{:?}", t);
    }

    #[test]
    fn test_to_owned() {
        let a = "hello";
        let b = "hello";

        let a1 = a.to_owned();
        let b1 = b.to_string();

        println!("{}", a == a1);
        println!("{}", b == b1);
    }

    #[test]
    fn test_into_string() {
        fn aa<T: Into<String>>(src: T) {
            println!("{}", src.into())
        }

        let a = "hello".to_string();
        aa(a);
        //println!("{}", a);
        let a = "hello".to_string();
        aa(&a);
        println!("{}", a);
    }

    #[test]
    fn test_cell() {
        // Cell: 是針對個 innter T 的型別操作，不能針對個別 element in T 的操作，
        // 如果要針對個別 element in T 的操作，要用 RefCell.

        use std::cell::{Cell, RefCell};

        // declare mut, 才能修改 inner value。
        // 但在 struct 如果要操作 inner value，因為不能 declare mut，所以不能用。
        // struct A {
        //   a: mut Cell<Vec<i32>>, // 錯誤的 declare.
        //}
        // 因此在此情況下，要用 RefCell.
        // 因為有此現像，才會有實作 Copy 用 Cell, 沒有用 RefCell 的假像。
        let mut a = Cell::new(vec![1, 2, 3]);
        a.get_mut().push(4);

        println!("{:?}", a.take());
        println!("{:?}", a.take());

        let a = Cell::new(vec![1, 2, 3]);
        let b = Cell::new(vec![4, 5, 6]);
        a.swap(&b);
        println!("{:?}", a.take());
        println!("{:?}", b.take());

        let a = RefCell::new(vec![1, 2, 3]);
        a.borrow_mut().push(4);
        println!("{:?}", a);
    }

    #[test]
    fn test_split() {
        let str = " hello world我有中文，哈哈哈 ".to_string();
        println!("{:?}", str.split("").collect::<Vec<&str>>());
        //println!("{:?}", str.split_inclusive("").collect::<Vec<&str>>());
    }

    #[test]
    fn test_str_index() {
        let a = "中文測試";
        println!("{}", a.chars().count());
        println!("{}", a.chars().nth(2).unwrap());
        println!("{:?}", &a.as_bytes()[0..3]);
        println!("{:?}", a.as_bytes()[0]);
        println!("{}", &a[0..3]);
        //println!("{}", &a[0..2]); // panic
    }

    #[test]
    fn test_string_index() {
        let a = "中文測試".to_string();
        println!("{}", a.chars().nth(2).unwrap());
        println!("{:?}", &a.as_bytes()[0..3]);
        println!("{:?}", a.as_bytes()[0]);
        println!("{}", &a[0..3]);
        //println!("{}", &a[0..2]); // panic
    }

    #[test]
    fn test_struct_refcell() {
        use std::cell::RefCell;
        use std::rc::Rc;

        #[derive(Debug)]
        struct A {
            is_true: Rc<RefCell<bool>>,
        }

        let a = A {
            is_true: Rc::new(RefCell::new(true)),
        };

        {
            // 要用 scope 的方式，才能在 scope 結束時，釋放 borrow_mut.
            // 否則會有 borrow_mut 重複的錯誤。
            let r2 = a.is_true.clone();
            let mut b2 = r2.borrow_mut();
            *b2 = false;
            println!("{:?}", b2);
        }

        println!("{:?}", a);
    }
}
