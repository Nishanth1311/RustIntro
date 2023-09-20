fn main() {
    // Variable types"
    // SCALAR data types
    let var_i8: i8 = 100;
    println!("{var_i8}");
    let var_i16: i16 = -1; // (0xFFFF)
    println!("{var_i16}");
    let var_u32: u32 = 0xFFFFFFFF;
    println!("{var_u32}");
    let var_u64: u64 = var_i16 as u64;
    println!("{var_u64}");
    let var_i128: i128 = 0x0123456789ABCDEF0123456789ABCDEF;
    println!("{var_i128}");
    let var_usize: usize = 10;
    println!("{var_usize}");

    let var_bool: bool = true;
    println!("{var_bool}");

    // Can convert chars only from u8
    let var_char: char = var_i8 as u8 as char;
    println!("{var_char}");

    // Floats must have a decimal point
    let var_f32: f32 = 12.15;
    println!("{var_f32}");

    let var_f64: f64 = 8.0;
    println!("{var_f64}");

    // COMPOUND DATA TYPES
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{} {} {}", tup.0, tup.1, tup.2);

    let arr = [1, 2, 3];
    println!("{} {} {}", arr[0], arr[1], arr[2]);

    // Mutability
    let mut mut_i32: i32 = 100;
    println!("{mut_i32}");
    mut_i32 = 25;
    println!("{mut_i32}");

    // Shadowing
    let shad_f32: f32 = 1.0;
    println!("{shad_f32}");
    let shad_f32: f32 = 1.0 + 2.9;
    println!("{shad_f32}");

    // CONST
    const CELSIUS_TO_KELVIN: i32 = 273;
    const KELVIN_TO_CELSIUS: i32 = -273;
    println!("{CELSIUS_TO_KELVIN}");
    println!("{KELVIN_TO_CELSIUS}");
    // cannot shadow consts
}
