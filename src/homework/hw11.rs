use rand::Rng;

// Функція генерації вектора зі значеннями від 10 до 99
pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

// Повертає мінімальну суму і індекси пари
pub fn min_adjacent_sum(data: &[i32]) -> (usize, i32) {
    let mut min_sum = i32::MAX;
    let mut min_index = 0;
    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }
    (min_index, min_sum)
}

// Функція виводу в форматі, як на зображенні
pub fn print_data_with_min_pair(data: &[i32]) {
    // Вивід індексів
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();

    // Вивід значень
    print!("data:    ");
    for val in data {
        print!("{:>3},", val);
    }
    println!();

    // Пошук мінімальної суми
    let (min_index, min_sum) = min_adjacent_sum(data);

    // Порожній рядок з відмітками
    print!("indexes: ");
    for i in 0..data.len() {
        if i == min_index {
            print!(" \\__ ");
        } else if i == min_index + 1 {
            print!("__/");
        } else {
            print!("     ");
        }
    }
    println!();

    // Виведення результату
    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[min_index],
        data[min_index + 1],
        min_sum,
        min_index,
        min_index + 1
    );
}
