fn draw_tree(levels: u32) {
    let mut width = 1;

    for level in 0..levels {
        for i in 0..(level + 3) {
            let stars = (0..(1 + 2 * i)).map(|_| '*').collect::<String>();
            let padding = (width + levels as usize * 2) - i as usize;
            println!("{:padding$}{}", "", stars, padding = padding);
        }
    }
}

fn main() {
    let levels = 4; // кількість трикутників
    draw_tree(levels);
}
