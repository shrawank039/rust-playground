pub fn binary(){

    // Binary
    let a: u8 = 0b1111_0000;
    let b: u8 = 0b0000_1111;

    println!("a's value is {}", a);
    println!("b's value is {}", b);

    // Logic Gates
    println!("AND: {:08b}", a & b);
    println!("OR: {:08b}", a | b);
    println!("XOR: {:08b}", a ^ b);
    println!("NOT: {:08b}", !a);

    // Bitwise
    println!("Shift Left: {:08b}", a << 1);
    println!(" a << 1 {} ", a << 1);
    println!("Shift Right: {:08b}", a >> 1);
    println!(" a >> 1 {} ", a >> 1);

    // Little Endian or Big Endian
    let n: u32 = 0x1234;
    println!("n is: {:?}", n);

    let big_endian = n.to_be_bytes();
    let little_endian = n.to_le_bytes();

    println!("n in big_endian: {:02X}{:02X}", big_endian[0], big_endian[1]);
    println!("n in little_endian: {:02X}{:02X}", little_endian[0], little_endian[1]);

}