const SIZE: usize = 14;

fn main() {
    let mut envelope = vec![vec![' '; SIZE * 2]; SIZE];
    
    // Малюємо рамку
    for i in 0..SIZE {
        envelope[0][i] = '*';
        envelope[0][SIZE * 2 - 1 - i] = '*';
        envelope[SIZE - 1][i] = '*';
        envelope[SIZE - 1][SIZE * 2 - 1 - i] = '*';
    }
    
    // Малюємо діагоналі
    for i in 1..SIZE - 1 {
        envelope[i][i] = '*';
        envelope[i][SIZE * 2 - 1 - i] = '*';
    }
    
    println!("{}", envelope.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<_>>().join("\n"));
}
