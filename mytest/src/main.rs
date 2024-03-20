use std::borrow::BorrowMut;

fn main() {
    // trait
    //{
    //    trait MyTrait {
    //        fn hello(&self) -> String;
    //    }
    //
    //    struct My1;
    //    struct My2;
    //
    //    impl MyTrait for My1 {
    //        fn hello(&self) -> String {
    //            "My1".to_string()
    //        }
    //    }
    //
    //    impl Drop for My1 {
    //        fn drop(&mut self) {
    //            println!("My1 drop");
    //        }
    //    }
    //
    //    impl MyTrait for My2 {
    //        fn hello(&self) -> String {
    //            "My2".to_string()
    //        }
    //    }
    //    impl Drop for My2 {
    //        fn drop(&mut self) {
    //            println!("My2 drop");
    //        }
    //    }
    //
    //    fn test_trait1(_: bool) -> impl MyTrait {
    //        My1
    //    }
    //
    //    fn test_trait2(a: bool) -> Box<impl MyTrait> {
    //        if a {
    //            Box::new(My2)
    //        } else {
    //            Box::new(My2)
    //        }
    //    }
    //
    //    fn test_trait3(a: bool) -> Box<dyn MyTrait> {
    //        if a {
    //            Box::new(My1)
    //        } else {
    //            Box::new(My2)
    //        }
    //    }
    //
    //    fn test_trait4(a: impl MyTrait) {
    //        println!("test_trait4:{:?}", a.hello());
    //    }
    //
    //    //fn test_trait5(a: dyn MyTrait) {
    //    //    println!("test_trait5:{:?}", a.hello());
    //    //}
    //
    //    fn test_trait_generic<T: MyTrait>(a: T) {
    //        println!("test_trait_generic:{:?}", a.hello());
    //    }
    //
    //    println!("test_trait1:{:?}", test_trait1(true).hello());
    //    println!("test_trait1:{:?}", test_trait1(false).hello());
    //
    //    println!("test_trait2:{:?}", test_trait2(true).hello());
    //    println!("test_trait2:{:?}", test_trait2(false).hello());
    //
    //    println!("test_trait3:{:?}", test_trait3(true).hello());
    //    println!("test_trait3:{:?}", test_trait3(false).hello());
    //
    //    test_trait4(My1);
    //    test_trait4(My2);
    //
    //    let mut smart_ptr: Box<dyn MyTrait> = Box::new(My1);
    //    println!("smart_ptr:{:?}", smart_ptr.hello());
    //    smart_ptr = Box::new(My2);
    //    println!("smart_ptr:{:?}", smart_ptr.hello());
    //
    //    test_trait_generic(My1);
    //    test_trait_generic(My2);
    //}

    // lifetime
    //{
    //
    //    struct My1<'a, 'b> {
    //        pub id: &'a str,
    //        pub name: &'b str,
    //
    //    }
    //
    //
    //    impl<'a, 'b> My1<'a, 'b> {
    //        fn new(id: &'a str, name: &'b str) -> Self {
    //            My1 { id, name }
    //        }
    //
    //        fn hello(&self) {
    //            println!("{} hello:{}", self.id, self.name);
    //        }
    //    }
    //    unsafe impl<'a ,'b> Send for My1<'a, 'b>{}
    //    unsafe impl<'a, 'b> Sync for My1<'a, 'b>{}
    //
    //    let id1 = "id1".to_string();
    //    {
    //        let name1 = "name1".to_string();
    //
    //        {
    //            let id1 = &id1;
    //            let name1 = &name1;
    //
    //            std::thread::spawn(move || {
    //                let my1 = My1::new(&id1, &name1);
    //                my1.hello();
    //            });
    //        }
    //    }
    //
    //}

    // ownership and borrow
    //{
    //    let mut a = 10;
    //    println!("a = {a}");
    //    fn test1(mut x: i32) -> i32 {
    //        x += x;
    //        x
    //    }
    //
    //    let b = test1(a);
    //    println!("a = {a}");
    //    println!("b = {b}");
    //
    //    a *= a;
    //
    //    fn test2(y: &mut i32) -> i32 {
    //        *y += *y;
    //        *y
    //    }
    //    println!("a = {a}");
    //    let c = test2(&mut a);
    //    println!("a = {a}");
    //    println!("c = {c}");
    //}

    // first class
    //{
    //    fn add_one(x: i32) -> i32 {
    //        x + 1
    //    }
    //
    //    fn test(x: i32, f: fn(i32) -> i32) -> i32 {
    //        f(x)
    //    }
    //
    //    let a = 10;
    //    let b = test(a, add_one);
    //
    //    println!("a = {a}, b = {b}");
    //}

    // Unit
    //{
    //
    //    fn test1() -> () {
    //        println!("test1");
    //    }
    //
    //    fn test2() -> () {
    //        println!("test2")
    //    }
    //
    //    test1();
    //    test2();
    //}

    {
        let mut s = String::from("hello");
        s.push_str(" world");
        let ms = &s[0..2];
        //s = String::from("w");
        //    |
        // let ms = &s[0..2];
        //           - `s` is borrowed here
        // s = String::from("w");
        // ^ `s` is assigned to here but it was already borrowed

        // println!("ms = {}", ms);
        //                     -- borrow later used here

        println!("ms = {}", ms);

        //package main
        //
        //import "fmt"
        //
        //func main() {
        //	s := "hello"
        //	s1 := s[0:2]
        //	s = "x"
        //
        //	fmt.Println(s1)
        //	fmt.Println(s)
        //}

        // result:
        // he
        // x
    }

    //{
    //    let x = std::cell::RefCell::new(Some(10));
    //
    //    let y = x.borrow_mut().take();
    //
    //    println!("{x:?}, {y:?}");
    //}

    //{
    //    trait x {
    //        fn test(&self, a: String, b: &String) -> String;
    //    }
    //
    //    struct y;
    //
    //    impl x for y {
    //        fn test(&self, mut a: String, b: &String) -> String {
    //            // b.push('b');
    //            // b.push('b');
    //            // ^^^^^^^^^^^ `b` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    //            a.push_str(b);
    //            return a;
    //        }
    //    }
    //}
    //{
    //    fn a1(mut a: String) -> String {
    //        a.push_str(" a1");
    //        a
    //    }
    //    fn a2(a:&mut String) {
    //        a.push_str(" a2");
    //    }
    //
    //    let x1 = String::from("x1");
    //    let mut x1 = a1(x1);
    //    println!("{:?}", x1);
    //    a2(&mut x1);
    //    println!("{:?}", x1);
    //}
    //
    //{
    //    trait A {
    //        fn a(&self) {
    //            println!("trait default method");
    //        }
    //    }
    //
    //    struct B;
    //
    //    impl B {
    //        fn a(&self) {
    //            println!("overridden method");
    //            // call default method here
    //            A::a(self);
    //        }
    //    }
    //
    //    impl A for B {}
    //
    //    let a = B;
    //    a.a();
    //}
    //
    //
    //{
    //    trait A {
    //        fn a(&self) {
    //            println!("trait default method");
    //        }
    //    }
    //
    //    struct B {
    //        counter: std::cell::Cell<i32>,
    //    }
    //
    //    impl A for B {
    //        fn a(&self) {
    //            println!("overridden method {}", self.counter.get());
    //            self.counter.set(self.counter.get() + 1);
    //            if self.counter.get() > 5 {
    //                return;
    //            }
    //            // recursive call here.
    //            A::a(self);
    //        }
    //    }
    //
    //
    //
    //    let a = B {counter: std::cell::Cell::new(0),};
    //    a.a();
    //
    //}

    {
        trait Handler<B> {
            fn test(x: B);
        }

        struct A;

        impl Handler<i32> for A {
            fn test(x: i32) {}
        }
    }
}
