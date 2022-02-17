
#[cfg(test)]
mod tests {
    use std::num::Wrapping;

    const MAX_VALUE: i32 = 100;
    // const MAX_FOO: i32 = foo();

    #[test]
    #[should_panic]
    fn before() {
        let mut x = 3;
        println!("x is {}", x);
        x = 5;
        println!("x is {}", x);

        println!("max value: {}", MAX_VALUE);
        // println!("max value: {}", MAX_FOO);

        let x = "233";
        let num: i32 = x.parse().expect("Not number");
        println!("x is : {}", x);
        println!("num is :{}", num);

        let wrap = Wrapping(200u8);
        let wrap2 = Wrapping(56u8);
        println!("warp is : {}", (wrap + wrap2).0);

        let u1 = foo();
        let u2 = 56u8;

        println!("u8 add is : {}", (u1 + u2));
    }

    fn foo() -> u8 {
        200u8
    }

    #[test]
    fn tuple() {
        let tuple = (100, 200.0, 3.4);
        let (x, y, z) = tuple;
        assert_eq!(x, tuple.0);
        assert_eq!(y, tuple.1);
        assert_eq!(z, tuple.2);
    }

    #[test]
    #[should_panic]
    fn array() {
        let array:[i32; 3]= [1, 2, 3];
        assert_eq!(1, array[0]);
        assert_eq!(3, array[2]);

        let array = [3; 3];
        assert_eq!(3, array[1]);

        array[index()];
    }

    #[test]
    fn if_test() {
        let condition = true;
        let num = if condition {
            2
        } else {
            3
        };
        assert_eq!(2, num);
    }

    #[test]
    fn loop_test() {
        let mut count = 0;
        let result = loop {
            count += 1;

            if count == 10 {
                break count * 2;
            }
        };
        assert_eq!(20, result);
    }

    #[test]
    fn for_test() {
        let array = [3; 5];
        for element in array.iter() {
            println!("{}", element);
        }
    }

    #[test]
    fn for_range_test() {
        for number in (1..5).rev() {
            println!("{}", number);
        }
    }

    fn index() -> usize {
        10
    }
}