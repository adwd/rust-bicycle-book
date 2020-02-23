use std::collections::HashMap;

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    {
        let xx = 3;
        fn add2(x: i32, y: i32) -> i32 {
            x + y
        }
        let mut adder: fn(i32, i32) -> i32 = |x, y| x + y + y;
        adder = add;
        adder = add2;
        println!("Hello, world!, {}", adder(1, 2));
    }

    {
        let arr = [1, 2, 3];
        println!("arr: {:?}", arr);
        let arr1 = [3, 4, 5];
        if let Some(x) = arr1.first() {
            println!("{}", x);
        }
        // thread 'main' panicked at 'index 5 out of range for slice of length 3'
        // println!("{:?}", &arr[2..5]);
        let mut arr2 = [1, 2, 3];
        arr2.reverse();
    }

    {
        let mut m1 = HashMap::new();
        m1.insert("1", 3);
        m1.get("2");
        let m2 = vec![("a", 1), ("b", 2)]
            .into_iter()
            .collect::<HashMap<_, _>>();
    }

    {
        let s: &str = "42";
        assert_eq!(42.to_string(), s);
        assert_eq!(42.to_string(), "42".to_string());
        let r2 = "abc".parse::<i32>();
        let r3: Result<f32, _> = "abc".parse();
    }

    {
        let r = ..;
        assert_eq!(r, std::ops::RangeFull);
    }

    {
        fn get_from_array(arr: &[i32]) -> Option<i32> {
            println!("get from array");
            let i1 = arr.get(0)?;
            println!("get from array 2");
            let i2 = arr.get(3)?;
            println!("get from array 3");
            Some(i1 + i2)
        }

        println!("{:?}", get_from_array(&[1, 2, 3, 4]));
        println!("{:?}", get_from_array(&[1]));
    }

    {
        // 名前付きフィールド構造体
        struct Polygon {
            vertexes: Vec<(i32, i32)>,
            stroke_width: u8,
            fill: (u8, u8, u8),
        }

        // タプル構造体
        struct Triangle(Vertex, Vertex, Vertex);
        struct Vertex(i32, i32);

        // ユニット構造体
        struct UniqueValue;
        struct UniqueValue1 {};
        struct UniqueValue2();

        let triangle = Polygon {
            vertexes: vec![(0, 0), (3, 0), (2, 2)],
            fill: (255, 255, 255),
            stroke_width: 1,
        };

        fn new_polygon(vertexes: Vec<(i32, i32)>) -> Polygon {
            let stroke_width = 1;
            let fill = (0, 0, 0);
            Polygon {
                vertexes,
                stroke_width,
                fill,
            }
        }

        assert_eq!(triangle.vertexes[0], (0, 0));
        let Polygon { vertexes: vx, .. } = triangle;
        assert_eq!(vx[0], (0, 0));

        let triangle2 = new_polygon(vx);

        // レコードアップデート構文 (functional record update syntax)
        let triangle3 = Polygon {
            stroke_width: 5,
            ..triangle2
        };

        #[derive(Default, Debug)]
        struct Polygon2 {
            vertexes: Vec<(i32, i32)>,
            stroke_width: u8,
            fill: (u8, u8, u8),
        }

        let poly1: Polygon2 = Default::default();
        println!("{:?}", poly1);
        let poly2 = Polygon2 {
            vertexes: triangle3.vertexes,
            ..Default::default()
        };
        println!("{:?}", poly2);

        #[derive(Debug)]
        struct Polygon3 {
            vertexes: Vec<(i32, i32)>,
            stroke_width: u8,
            fill: (u8, u8, u8),
        }
        impl Default for Polygon3 {
            fn default() -> Self {
                Self {
                    stroke_width: 1,
                    vertexes: Default::default(),
                    fill: Default::default(),
                }
            }
        }
    }

    {
        struct Triangle(Vertex, Vertex, Vertex);
        struct Vertex(i32, i32);

        let vx0 = Vertex(0, 0);
        let vx1 = Vertex(0, 1);
        let triangle = Triangle(vx0, vx1, Vertex(2, 2));
        assert_eq!((triangle.0).0, 0);

        // newtype pattern
        // https://doc.rust-lang.org/1.0.0/style/features/types/newtype.html
        struct UserName(String);
        struct Id(u64);
        struct Timestamp(u64);
        // type User = (Id, UserName, Timestamp);
        struct User(Id, UserName, Timestamp);

        fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
            User(id, name, created)
        }

        let id = Id(400);
        let now = Timestamp(4567890123);

        // nowとidの順番を間違えるとコンパイルエラーになってくれる
        // let bad_user = new_user(UserName(String::from("kazuki")), now, id);
        // error[E0308]: mismatched types
        // expected type `Id`, found type `Timestamp`
    }

    {
        // enumのバリアントには構造体と同じ文法でフィールドをもたせられる
        type UserName = String;
        #[derive(Debug)]
        enum Task {
            Open,
            AssignedTo(UserName),
            Working {
                assignee: UserName,
                remaining_hours: u16,
            },
            Done,
        }

        // use宣言でTaskが持つバリアントをインポートするとバリアント名が直接書けるようになる
        use Task::*;

        let tasks = vec![
            AssignedTo(String::from("junko")),
            Working {
                assignee: String::from("hiro"),
                remaining_hours: 18,
            },
            Done,
        ];

        tasks.iter().enumerate().for_each(|(i, task)| match task {
            AssignedTo(assignee) => println!("タスク{}は{}さんにアサインされています", i, assignee),
            Working {
                assignee,
                remaining_hours,
            } => println!(
                "タスク{}は{}さんが作業中です。残り{}時間の見込み",
                i, assignee, remaining_hours
            ),
            _ => println!("タスク{}はその他のステータス（{:?}）です", i, task),
        })
    }

    {
        42 as f64;
        let v: Vec<u8> = From::from("hello");
    }
}
