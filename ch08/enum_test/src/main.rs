fn main() {
    // {
    //     #[derive(Debug)]
    //     enum Direction {
    //         Up = 3,
    //         Down,
    //         Left,
    //         Right,
    //     }

    //     fn show(d: Direction) {
    //         println!("{:?}", d);
    //     }

    //     fn show2(d: &Direction) {
    //         println!("{:?}", d);
    //     }

    //     let up = Direction::Up;
    //     let down = Direction::Down;
    //     let left = Direction::Left;
    //     let right = Direction::Right;

    //     show(up);
    //     show2(&down);
    //     show(left);
    //     show2(&right);

    //     println!("{:?}", down);
    //     //println!("{:?}", up); // Error

    //     unsafe {
    //         println!("size_of: {}", std::mem::size_of::<Direction>());
    //         println!(
    //             "Up: {}",
    //             std::mem::transmute::<Direction, u8>(Direction::Up)
    //         );
    //         println!(
    //             "Down: {}",
    //             std::mem::transmute::<Direction, u8>(Direction::Down)
    //         );
    //         println!(
    //             "Left: {}",
    //             std::mem::transmute::<Direction, u8>(Direction::Left)
    //         );
    //         println!(
    //             "Down: {}",
    //             std::mem::transmute::<Direction, u8>(Direction::Right)
    //         );
    //     }
    // }

    // {
    //     struct Empty;

    //     println!("size of Empty: {}", std::mem::size_of::<Empty>());

    //     #[derive(Debug)]
    //     struct Point {
    //         x: i32,
    //         y: i32,
    //     }

    //     #[derive(Debug)]
    //     enum Polygon {
    //         Triangle(Point, Point, Point),
    //         Rectangle(Point, Point, Point, Point),
    //     }

    //     let p1 = Polygon::Triangle(
    //         Point { x: 0, y: 0 },
    //         Point { x: 1, y: 0 },
    //         Point { x: 0, y: 1 },
    //     );

    //     let p2 = Polygon::Rectangle(
    //         Point { x: 0, y: 0 },
    //         Point { x: 1, y: 0 },
    //         Point { x: 1, y: 1 },
    //         Point { x: 0, y: 1 },
    //     );

    //     println!("{:?}", p1);
    //     println!("{:?}", p2);

    //     println!("size of Point: {}", std::mem::size_of::<Point>());
    //     println!("size of Polygon: {}", std::mem::size_of::<Polygon>());
    //     println!("size of Polygon::Triangle: {}", std::mem::size_of_val(&p1));
    //     println!("size of Polygon::Rectangle: {}", std::mem::size_of_val(&p2));
    // }

    // {
    //     struct Empty;

    //     #[derive(Debug)]
    //     struct Point {
    //         x: i32,
    //         y: i32,
    //     }

    //     #[derive(Debug)]
    //     struct Triangle(Point, Point, Point);

    //     #[derive(Debug)]
    //     struct Rectangle(Point, Point, Point, Point);

    //     #[derive(Debug)]
    //     enum Polygon {
    //         Triangle(Point, Point, Point),
    //         Rectangle(Point, Point, Point, Point),
    //     }

    //     let p1 = Polygon::Triangle(
    //         Point { x: 0, y: 0 },
    //         Point { x: 1, y: 0 },
    //         Point { x: 0, y: 1 },
    //     );

    //     let p2 = Polygon::Rectangle(
    //         Point { x: 0, y: 0 },
    //         Point { x: 1, y: 0 },
    //         Point { x: 1, y: 1 },
    //         Point { x: 0, y: 1 },
    //     );

    //     println!("size of Empty: {}", std::mem::size_of::<Empty>()); // 0
    //     println!("size of Point: {}", std::mem::size_of::<Point>()); // 8
    //     println!("size of Triangle: {}", std::mem::size_of::<Triangle>()); // 24
    //     println!("size of Rectangle: {}", std::mem::size_of::<Rectangle>()); // 32
    //     println!("size of Polygon: {}", std::mem::size_of::<Polygon>()); // 36
    //     println!("size of Polygon::Triangle: {}", std::mem::size_of_val(&p1)); // 36
    //     println!("size of Polygon::Rectangle: {}", std::mem::size_of_val(&p2)); // 36
    // }

    // {
    //     enum Corner {
    //         LeftTop,
    //         LeftBottom,
    //         RightTop,
    //         RightBottom,
    //     }

    //     println!("size of Corner: {}", std::mem::size_of::<Corner>()); // 1

    //     unsafe {
    //         println!(
    //             "Corner::LeftTop is {}",
    //             std::mem::transmute::<Corner, u8>(Corner::LeftTop)
    //         ); // 0

    //         println!(
    //             "Corner::LeftBottom is {}",
    //             std::mem::transmute::<Corner, u8>(Corner::LeftBottom)
    //         ); // 1

    //         println!(
    //             "Corner::RightTop is {}",
    //             std::mem::transmute::<Corner, u8>(Corner::RightTop)
    //         ); // 2

    //         println!(
    //             "Corner::RightBottom is {}",
    //             std::mem::transmute::<Corner, u8>(Corner::RightBottom)
    //         ); // 3
    //     }

    //     #[derive(Debug)]
    //     #[repr(i32)]
    //     enum Direction {
    //         Up = 3,
    //         Down,
    //         Left,
    //         Right,
    //     }

    //     println!("size of Direction: {}", std::mem::size_of::<Direction>()); // 4
    //     unsafe {
    //         println!(
    //             "Direction::Up is {}",
    //             std::mem::transmute::<Direction, i32>(Direction::Up)
    //         ); // 3

    //         println!(
    //             "Direction::Down is {}",
    //             std::mem::transmute::<Direction, i32>(Direction::Down)
    //         ); // 4

    //         println!(
    //             "Direction::Left is {}",
    //             std::mem::transmute::<Direction, i32>(Direction::Left)
    //         ); // 5

    //         println!(
    //             "Direction::Right is {}",
    //             std::mem::transmute::<Direction, i32>(Direction::Right)
    //         ); // 6
    //     }
    // }

    {
        #[derive(Debug)]
        enum Either<L, R> {
            Left(L),
            Right(R),
        }

        impl<L, R> Either<L, R> {
            fn is_right(&self) -> bool {
                match self {
                    Either::Right(_) => true,
                    _ => false,
                }
            }

            fn is_left(&self) -> bool {
                !self.is_right()
            }

            fn get_right(&self) -> Option<&R> {
                match self {
                    Either::Right(r) => Some(r),
                    _ => None,
                }
            }

            fn get_left(&self) -> Option<&L> {
                match self {
                    Either::Left(l) => Some(l),
                    _ => None,
                }
            }

            fn to_option(self) -> Option<R> {
                match self {
                    Either::Right(r) => Some(r),
                    _ => None,
                }
            }
        }

        let e1: Either<i32, ()> = Either::Left(1);
        let e2: Either<(), i32> = Either::Right(2);

        println!(
            "{:?}, {}, {}, {:?}, {:?}",
            e1,
            e1.is_left(),
            e1.is_right(),
            e1.get_left(),
            e1.get_right(),
        );
        println!("{:?}", e1.to_option());

        println!(
            "{:?}, {}, {}, {:?}, {:?}",
            e2,
            e2.is_left(),
            e2.is_right(),
            e2.get_left(),
            e2.get_right(),
        );
        println!("{:?}", e2.to_option());
    }
}
