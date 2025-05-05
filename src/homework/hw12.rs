pub fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len();

    if total as usize % n != 0 {
        // Неможливо зрівняти вантаж
        return usize::MAX;
    }

    let target = total / n as u32;
    let mut moves = 0;

    for &s in shipments {
        if s > target {
            moves += (s - target) as usize;
        }
    }

    moves
}
