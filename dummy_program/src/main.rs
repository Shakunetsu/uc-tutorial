use windows::{
    Win32::System::Threading::*,
};

fn main() {
    let var_int: u32 = 123456;
    let var_string: &str = "DefaultString";
    let text: &str = "Long char array right there ->";
    let mut arr_char: [char; 128] = [' '; 128];
    for (i, c) in text.chars().enumerate() {
        arr_char[i] = c;
    }
}
