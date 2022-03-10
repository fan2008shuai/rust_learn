
#[cfg(test)]
mod test {

    #[derive(Debug)]
    struct User<'a> {
        name: String,
        email: String,
        age: i32,
        active: bool,
        content: &'a str
    }

    #[test]
    fn struct_test() {
        let name = String::from("hello");

        let novel = String::from("call me hello. some years ago...");
        let content = novel.split(".").next().expect("couldn't find a '.'");

        let user1 = User {
            email: String::from("world@111.com"),
            age: 100,
            active: true,
            name,
            content
        };
        // move error
        // println!("origin: {}", name);

        // it's ok
        println!("content: {}", content);

        println!{"{:?}", user1};

        assert_eq!("hello", user1.name);

        let user2 = User {
            name: String::from("hello2"),
            ..user1
        };
        println!("{:?}", user2)
    }

    // 元组结构体
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    #[test]
    fn tuple_struct_test() {
        let color = Color(1, 2, 3);
        let Point(_p1, p2, _p3) = Point(1, 2, 3);
        assert_eq!(1, color.0);
        assert_eq!(2, p2);
    }

    // 空结构体
    struct Empty;

    impl Clone for Empty {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl Copy for Empty {

    }

}

