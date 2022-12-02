macro_rules! input {
    ($x:literal) => {
        include_str!(concat!("./../days/", $x, ".txt"))
    };
}
#[allow(unused_macros)]
macro_rules! input_to_i32_iterator {
    ($x:literal) => {
        input!($x).split("\n").map(|x| x.parse::<i32>().unwrap())
    };
}
#[allow(unused_macros)]
macro_rules! input_to_2d_i32_vec {
    ($x:literal) => {
        input!($x).split("\n").collect::<Vec<_>>()[..]
            .split(|x| *x == "")
            .map(|y| {
                y.iter()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    };
}
#[allow(unused_macros)]
macro_rules! input_to_i32_comma_iterator {
    ($x:literal) => {
        input!($x).split(",").map(|x| x.parse::<i32>().unwrap())
    };
}
#[allow(unused_macros)]
macro_rules! input_to_str_iterator {
    ($x:literal) => {
        input!($x).split("\n")
    };
}
#[allow(unused_macros)]
macro_rules! input_to_i32_vec {
    ($x:literal) => {
        input_to_i32_iterator!($x).collect::<Vec<_>>()
    };
}
#[allow(unused_macros)]
macro_rules! input_to_i32_comma_vec {
    ($x:literal) => {
        input_to_i32_comma_iterator!($x).collect::<Vec<_>>()
    };
}
#[allow(unused_macros)]
macro_rules! input_to_str_vec {
    ($x:literal) => {
        input_to_str_iterator!($x).collect::<Vec<_>>()
    };
}
macro_rules! part1 {
    ($x:expr) => {
        println!("PART 1: {:?}", $x)
    };
}
macro_rules! part2 {
    ($x:expr) => {
        println!("PART 2: {:?}", $x)
    };
}
