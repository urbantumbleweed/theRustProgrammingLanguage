use std::io;

fn main() {
    let _int_8: i8 = 127;
    let _int_8_negative: i8 = -128;
    let _int_16: i16 = 32_767;
    let _int_16_negative: i16 = -32_768;
    let _int_32: i32 = 2_147_483_647;
    let _int_32_negative: i32 = -2_147_483_648;
    let _int_64: i64 = 9_223_372_036_854_775_807;
    let _int_64_negative: i64 = -922_337_203_685_4775_808;
    let _uint_8: u8 = 255;
    let _uint_16: u16 = 65_535;
    let _uint_32: u32 = 4_294_967_295;
    let _uint_64: u64 = 18_446_744_073_709_551_615;
    let _uu: usize = 100;

    let _float_32: f32 = 0.1;
    let _float_64: f64 = 0.1;

    let _char: char = 'R';
    let _bool: bool = true;
    let _bool_negative: bool = false;

    let _tuple: (i32, f64, char, bool) = (5, 0.1, 'R', true);
    let _arr: [i32; 5] = [1, 2, 3, 4, 5];
    let array_same: [u8; 26] = [0; 26];
    let _sl: &[i32] = &[1, 2, 3, 4, 5];
    let _sl_mut: &mut [i32] = &mut [1, 2, 3, 4, 5];

    let (five, float, r_char, t_bool) = _tuple;

    let mut index: String = String::new();

    io::stdin().read_line(&mut index).expect("failed to read line");
    let index: usize = index.trim().parse().expect("index was not a number");

    println!("the array_sameis {:?}", array_same);
    println!("out of bounds {:?}", _arr[index]);
    println!("five: {:?}, float: {:?}, r_char: {:?}, t_bool: {:?}", five, float, r_char, t_bool);
    another_function(&_arr[2..]);
    noop();

}

fn another_function(param: &[i32]) {
    println!("another function {:?}", param);
}

fn noop() -> () {
    let x: i32 = 53;
    println!("in the noop function {:?}", x);
    ()
}

