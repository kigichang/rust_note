fn main() {
    // {
    //     let x = (1_i32, true); // declare a tuple
    //     println!("{:?}", x); // (1, true)
    //     assert_eq!(x.0, 1_i32); // 取得 tuple 的第一個元素
    //     assert_eq!(x.1, true); // 取得 tuple 的第二個元素

    //     let (a, b) = x; // destructuring
    //     println!("a = {}, b = {}", a, b); // a = 1, b = true

    //     let x = 1_i32;
    //     let y = 2_i32;
    //     println!("x = {}, y = {}", x, y); // x = 1, y = 2
    //     let (y, x) = (x, y); // swap x and y
    //     println!("x = {}, y = {}", x, y); // x = 2, y = 1
    // }

    // {
    //     let arr = [0_i32; 10]; // 宣告陣列，給予初始值與長度
    //     println!("{:?}", arr); // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    //     let arr = [1, 2, 3, 4, 5]; // 宣告陳列
    //     println!("{:?}", arr); // [1, 2, 3, 4, 5]
    //     println!("length of arr = {}", arr.len()); // 陣列長度
    //     println!("first element = {}", arr[0]); // 取得陣列元素

    //     let arr2 = arr.map(|x| x * 2); // map 操作
    //     println!("{:?}", arr); // [1, 2, 3, 4, 5]
    //     println!("{:?}", arr2); // [2, 4, 6, 8, 10]

    //     let slice = &arr[1..3]; // 取得 Slice reference
    //     println!("{:?}", slice); // [2, 3]

    //     let slice = &arr[1..=3]; // 取得 Slice reference
    //     println!("{:?}", slice); //[2, 3, 4]
    // }

    {
        let v = vec![1, 2, 3, 4, 5]; // v 在這裡被創建
        assert!(!v.is_empty());

        let v = vec![1; 5]; // 長度為 5 的向量，每個元素都是 1
        assert_eq!(v, vec![1, 1, 1, 1, 1]);

        let v: Vec<i32> = Vec::new(); // 空的 vector
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 0);

        let v = [1, 2, 3].to_vec();
        assert_eq!(v.len(), 3);
        assert_eq!(v.capacity(), 3);
        assert!(!v.is_empty());

        let v: Vec<i32> = Vec::with_capacity(10);
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 10);
        assert!(v.is_empty());

        let v = vec!["a", "b", "c"];
        assert_eq!(v[0], "a");
        assert_eq!(v[2], "c");

        let v: &[i32] = &(vec![1, 2, 3, 4, 5][1..4]); // 取得 Slice reference
        assert_eq!(v, &[2, 3, 4]);
    }

    {
        let mut v = vec![1, 2, 3, 4, 5];
        v.push(6);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(v.pop(), Some(6));
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        v.insert(0, -1);
        assert_eq!(v, [-1, 1, 2, 3, 4, 5]);
        v.insert(v.len(), 6);
        assert_eq!(v, [-1, 1, 2, 3, 4, 5, 6]);

        assert_eq!(v.remove(0), -1);
        assert_eq!(v, [1, 2, 3, 4, 5, 6]);
        assert_eq!(v.remove(v.len() - 1), 6);
        assert_eq!(v, [1, 2, 3, 4, 5]);

        v[0] = 0;
        assert_eq!(v, [0, 2, 3, 4, 5]);

        v[0] = 1;
        let new = vec![7, 8, 9];
        let u = v.splice(1..=2, new).collect::<Vec<_>>();
        assert_eq!(u, vec![2, 3]);
        assert_eq!(v, vec![1, 7, 8, 9, 4, 5]);

        v.clear();
        assert!(v.is_empty());
    }

    {
        let v = vec![1, 2, 3, 4, 5];
        println!("len: {}", v.len());
        println!("cap: {}", v.capacity());

        let mut v = Vec::new();
        let mut cap = v.capacity();
        for i in 0..100 {
            v.push(i);
            if cap != v.capacity() {
                println!("{}: cap: {} -> {}", i, cap, v.capacity());
                cap = v.capacity();
            }
        }
        println!("length = {}, capacity = {}", v.len(), v.capacity()); // length = 100, capacity = 128

        println!("with capacity");
        let mut v = Vec::with_capacity(100);
        let mut cap = v.capacity();
        for i in 0..100 {
            v.push(i);
            if cap != v.capacity() {
                println!("{}: cap: {} -> {}", i, cap, v.capacity());
                cap = v.capacity();
            }
        }
        println!("length = {}, capacity = {}", v.len(), v.capacity()); // length = 100, capacity = 100
    }

    {
        let v = vec![1, 2, 3, 4, 5];
        for x in &v {
            println!("{}", x);
        }

        let it = v.iter();
        for x in it {
            println!("{}", x);
        }

        // for x in v {
        //     println!("{}", x);
        // }
        // println!("{:?}", v);
    }

    {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        println!("window");
        let window = v.windows(4);
        for w in window {
            println!("{:?}", w);
        }

        println!("chunks");
        let chunk = v.chunks(4);
        for c in chunk {
            println!("{:?}", c);
        }

        println!("defined on line: {}", line!());
        let a = 10;
        let b = dbg!(a * 2) + 10;
        println!("{}", b);
        assert_eq!(b, 30);
    }
}
