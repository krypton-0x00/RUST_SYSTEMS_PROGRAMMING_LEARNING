fn main() {
    let n: f32 = 42.42;

    let n_bits: u32 = n.to_bits();
    println!("BIT REPRESENTATION OF {n}: {n_bits}");

    let sign_bit = n_bits >> 31;
    //if the number was -ve sign_bit will be 1;
    println!("SIGN_BIT OF {n}: {sign_bit}");

    //shift by 23 gets rid of mantissa,
    //now we only have the signbit and the exponent bit,
    //then we &and it with the 0xff or 11111111 it will keep only 8bits which are 1,
    //now we have the exponent_bit;
    let exponent_bit = (((n_bits >> 23) & 0xff) as i32) - 127;
    println!("EXPONENT OF {n} : {exponent_bit}");

    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = n_bits & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    println!("MANTISSA OF {n}: {mantissa}");
}
