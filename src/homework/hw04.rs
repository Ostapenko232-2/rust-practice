const WIDTH: usize = 5; // Половина ширини ромба
const HEIGHT: usize = 6; // Висота верхньої частини ромба

fn main() {
    let mut output = String::new();
    
    // Верхня частина ромба
    for i in 0..HEIGHT {
        output.push_str(&" ".repeat(WIDTH - i));
        output.push_str(&"*".repeat(2 * i + 1));
        output.push('\n');
    }
    
    // Нижня частина ромба
    for i in (0..HEIGHT - 1).rev() {
        output.push_str(&" ".repeat(WIDTH - i));
        output.push_str(&"*".repeat(2 * i + 1));
        output.push('\n');
    }
    
    print!("{}", output);
}
