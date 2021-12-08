macro_rules! input {
    ($x:literal) => {
        include_str!(concat!("./../days/", $x, ".txt"))
    };
}

macro_rules! input_to_i32_iterator {
    ($x:literal) => {
        input!($x).split("\n").map(|x| x.parse::<i32>().unwrap())
    };
}
macro_rules! input_to_i32_comma_iterator {
    ($x:literal) => {
        input!($x).split(",").map(|x| x.parse::<i32>().unwrap())
    };
}
macro_rules! input_to_str_iterator {
    ($x:literal) => {
        input!($x).split("\n")
    };
}

macro_rules! input_to_i32_vec {
    ($x:literal) => {
        input_to_i32_iterator!($x).collect::<Vec<_>>()
    };
}
macro_rules! input_to_i32_comma_vec {
    ($x:literal) => {
        input_to_i32_comma_iterator!($x).collect::<Vec<_>>()
    };
}
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
