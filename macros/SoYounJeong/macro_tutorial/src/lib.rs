

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn custom_vec_should_work() {
        use super::custom_vec;
        let test_5 = custom_vec!(i32, 0; 1);

        assert_eq!(test_5,[0]);
    } 

    #[test]
    fn custom_vec2_should_work() {
        use super::custom_vec2;
        let test = custom_vec2!(i32);
        assert_eq!(test, [])
    }
}

#[macro_export]
macro_rules! scanline {
    
    ($x: expr) => {
        std::io::stdin().read_line(&mut $x).unwrap();
    };
}

#[macro_export]
macro_rules! custom_println_tt {
    ($format: literal, $($token: tt)*) => {
        println!($format, $($token)*)
    };

    ($format: literal) => {
        println!($format)
    };
}

#[macro_export]
macro_rules! custom_println_expr {
    ($format: literal, $($token: expr), *) => {
        println!($format, $($token), * ) 
    }
}

#[macro_export]
macro_rules! custom_vec {
    ($t: ty) => {
        Vec::<$t>::new()
    };
    ($t: ty, $($e: expr), *) => {
        {
            let mut temp_vec = Vec::<$t>::new();
            $(
                temp_vec.push($e);
            )*
            temp_vec
        }
    };

    ($t: ty, $e: expr; $n: expr) => {
        {
            let mut temp_vec = Vec::<$t>::new();
            for _ in 0..$n {
                temp_vec.push($e);
            }
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! custom_vec2 {
    ($t: ty) => {
        {
            let v = Vec::<$t>::new();
            v
        }
    }
}