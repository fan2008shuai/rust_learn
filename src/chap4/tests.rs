
#[cfg(test)]
mod tests {

    #[test]
    fn mem_malloc_test() {
        let s1 = String::from("hello");
        println!("before s1: {}", s1);

        let s2 = s1;
        // println!("after: {}", s1);
        println!("after s2: {}", s2);
    }

    #[test]
    fn clone_test() {
        let s1 = String::from("hello");
        println!("before s1: {}", s1);

        let s2 = s1.clone();
        println!("after, s1: {}, s2: {}", s1, s2);
    }

    #[test]
    fn stack_test() {
        let x = 2;
        let y = x;
        println!("x: {}, y: {}", x, y);
    }

    #[test]
    fn copy_test() {

        #[derive(Debug)]
        // #[derive(Debug, Clone, Copy)]
        struct Hello {
            id: i32,
            age: i32,
        }

        impl Clone for Hello {
            fn clone(&self) -> Self {
                *self
                // Hello {
                //     id: self.id,
                //     age: self.age
                // }
            }
        }

        impl Copy for Hello {
        }

        let hello1 = Hello {
            id: 1,
            age: 1
        };

        let hello2 = hello1;
        println!("hello1: {:?}, hello2: {:?}", hello1, hello2);

    }

    #[test]
    fn parameter_return_test() {
        let s = String::from("hello");
        take_ownership(s); // s被移动到函数中，后面不可再访问
        // println!("s: {}", s);

        let x = 2;
        make_copy(x);
        println!("x: {}", x);

        let s = String::from("world");
        let s = take_and_gives_back(s);
        println!("s: {}", s);

        // 为了计算后继续使用s，不得不再返回一次
        let (s, length) = calculate_length(s);
        println!("s: {}, length: {}", s, length);
    }

    fn take_ownership(s: String) {
        println!("s: {}", s);
    } // s离开作用后，调用drop函数被释放

    fn make_copy(number: i32) {
        println!("number: {}", number);
    }

    fn take_and_gives_back(s: String) -> String {
        println!("s in take_and_gives_back: {}", s);
        s
    } // s被移动至调用函数

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len();
        (s, length)
    }

    #[test]
    fn borrowing_test() {
        let s = String::from("hello");
        let len = borrowing_calculate_length(&s);
        println!("s: {}, length: {}", s, len);

        let mut s1 = String::from("hello");
        borrowing_change(&mut s1);
        println!("s1: {}", s1);
    }

    fn borrowing_calculate_length(s: &String) -> usize {
        s.len()
    } // s虽然离开作用域，但是没有所有权，不会调用drop释放内存。

    fn borrowing_change(s: &mut String) {
        s.push_str(" world");
    }

    #[test]
    fn dangle_test() {
        let s = dangle2();
        println!("{}", s);
    }

    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //
    //     &s
    // }

    fn dangle2() -> String {
        let s = String::from("hello");

        s
    }

    #[test]
    fn life_cycle_test() {
        // ok
        let s1 = String::from("hello world");
        let s2 = String::from("hello");
        let result = longest(&s1, &s2);
        println!("longest: {}", result);

        // let s1 = "hello world";
        // let result;
        // {
        //     let s2 = String::from("hello");
        //     result = longest(s1, &s2);
        // }
        // println!("longest: {}", result);
    }

    // fn longest1(s1: &String, s2: &String) -> &String {
    //     if s1.len() > s2.len() {
    //         s1
    //     } else {
    //         s2
    //     }
    // }

    fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() > s2.len() {
            s1
        } else {
            s2
        }
    }


    #[test]
    fn slice_test() {
        let mut s = String::from("hello world!");
        let slice = search(&s);
        //s.clear();
        println!("{}", slice);
        s.clear();
    }

    #[test]
    fn slice_test2() {
        let s = "hello world";
        let slice = search(s);
        println!("{}", slice);
    }

    // 没有标注生命周期也可以编译通过
    fn search(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
        &s[..]
    }

}