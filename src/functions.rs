/// This is a doc comment #1
/// Function that adds five to a number
/// 
/// # Arguments (num: u32)
/// 
/// Returns u32
/// 
/// # Examples
/// ```
/// let x = 5;
/// let y = add_five(x);
/// ```
/// 
/**
 * This is a doc comment #2
 * 
 */

pub fn _playground(){
    // Works
    let x: i32 = 50;
    let y: i32 = x;
    println!("{}", x);
    println!("{}", y);

    // // Will not work
    // let s: String = String::from("Hello");
    // let t: String = s;
    // println!("{}", s);

    // Works 
    let s: String = String::from("Hello");
    let t: &String = &s.clone();
    println!("{} is clone", t);

    // Works 
    let s: String = String::from("Hello");
    let t: &String = &s;
    println!("{} is borrowed", t);

    // Does not work
    // let r: &String = make_string_dangle();

    // Works
    let r: String = make_string_not_dangle();
    println!("{} is not dangle", r);
}

// fn make_string_dangle() -> &String {
//     let s: String = String::from("Dangle");
//     let r: &String = &s;
//     r
// }

fn make_string_not_dangle() -> String {
    String::from("Dangle")
}

pub fn add_five(num: u32) -> u32{
    /*
    This is a Multi-line comment
    */
    num + 5
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_five_test() {
        // test add_five
        assert_eq!(add_five(5), 10);
    }
}