fn main() {
    // {
    //     struct Point {
    //         x: i32,
    //         y: i32,
    //     }

    //     struct Point3D(i32, i32, i32);

    //     let p = Point { x: 0, y: 0 };
    //     let p3d = Point3D(0, 0, 0);

    //     println!("({}, {})", p.x, p.y);
    //     println!("({}, {}, {})", p3d.0, p3d.1, p3d.2);
    // }

    // {
    //     #[derive(Debug)]
    //     struct Point {
    //         x: i32,
    //         y: i32,
    //     }

    //     let p = Point { x: 0, y: 0 };

    //     println!("({}, {})", p.x, p.y);
    //     println!("{:?}", p);
    // }

    // {
    //     #[derive(Debug)]
    //     struct Point;
    //     let p = Point;
    //     println!("{:?}", p);
    // }

    // {
    //     struct Point(i32, i32);
    //     let p = Point(0, 0);
    //     println!("({}, {})", p.0, p.1);
    // }

    // {
    //     #[derive(Debug)]
    //     struct Point(i32);

    //     #[derive(Debug)]
    //     struct Point3D(Point, Point, Point);

    //     let p3d = Point3D(Point(0), Point(0), Point(0));
    //     println!("{:?}", p3d);
    // }

    // {
    //     #[derive(Debug)]
    //     struct Point {
    //         x: i32,
    //         y: i32,
    //     }

    //     impl Point {
    //         fn new(x: i32, y: i32) -> Self {
    //             Point { x, y }
    //         }

    //         fn origin() -> Point {
    //             Point { x: 0, y: 0 }
    //         }

    //         fn distance(&self, p: &Point) -> f64 {
    //             let x = self.x - p.x;
    //             let y = self.y - p.y;
    //             ((x * x + y * y) as f64).sqrt()
    //         }

    //         fn move_to(&mut self, x: i32, y: i32) {
    //             self.x = x;
    //             self.y = y;
    //         }
    //     }

    //     impl std::ops::Add<(i32, i32)> for Point {
    //         type Output = Point;

    //         fn add(self, (x, y): (i32, i32)) -> Point {
    //             Point {
    //                 x: self.x + x,
    //                 y: self.y + y,
    //             }
    //         }
    //     }

    //     let p0 = Point::origin();
    //     let p1 = Point::new(20, 30);

    //     println!("{:?}", p0);
    //     println!("{:?}", p1);

    //     println!("distance from origin is {}", p1.distance(&p0));

    //     let mut p2 = Point::new(10, 10);
    //     p2.move_to(20, 5);
    //     assert_eq!(p2.x, 20);
    //     assert_eq!(p2.y, 5);

    //     let p3 = Point::new(10, 10) + (20, 5);
    //     println!("{:?}", p3);

    //     impl Point {
    //         fn add(self, (x, y): (i32, i32)) -> Self {
    //             Point {
    //                 x: self.x + x,
    //                 y: self.y + y,
    //             }
    //         }
    //     }

    //     let p1 = Point::new(10, 10);
    //     let p2 = p1.add((20, 5));
    //     println!("{:?}", p2);
    //     //println!("{:?}", p1); // Error
    // }
    //

    {
        use std::cell::{Cell, RefCell};

        #[derive(Debug)]
        struct Object {
            id: String,
            counter: Cell<u32>,
            flag1: Cell<Option<u32>>,
            flag2: RefCell<Option<u32>>,
            recorder: RefCell<Vec<u32>>,
        }

        impl Object {
            fn new(id: &str) -> Object {
                Object {
                    id: id.to_string(),
                    counter: Cell::new(0),
                    flag1: Cell::new(None),
                    flag2: RefCell::new(Some(1)),
                    recorder: RefCell::new(Vec::new()),
                }
            }

            fn count(&self) {
                self.counter.set(self.counter.get() + 1);
            }

            fn double_count(&self) -> u32 {
                self.count();
                self.counter.replace(self.counter.get() + 1)
            }

            fn toggle_flag(&self) {
                self.count();
                let counter = self.counter.get();
                if counter % 2 == 0 {
                    self.flag1.set(None);
                    self.flag2.replace(Some(counter));
                } else {
                    self.flag1.set(Some(counter));
                    self.flag2.borrow_mut().take();
                }
                self.recorder.borrow_mut().push(counter);
            }
        }

        let obj = Object::new("obj");
        println!("{:?}", obj);
        obj.toggle_flag();
        println!("{:?}", obj);
        obj.toggle_flag();
        println!("{:?}", obj);
        println!("old counter: {}", obj.double_count());
        println!("{:?}", obj);
        obj.toggle_flag();
        println!("{:?}", obj);
    }
}
