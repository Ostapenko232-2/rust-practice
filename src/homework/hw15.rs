use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let letters = ['m', 'u', 'x', 'a', 's', 'l', 'o', 'n'];
    let digits = (1..=8).collect::<Vec<_>>();
    let mut count = 0;

    for perm in digits.iter().permutations(8).unique() {
        let mut map = HashMap::new();
        for (&ch, &digit) in letters.iter().zip(perm.iter()) {
            map.insert(ch, digit);
        }

        let muxa = 1000 * map[&'m'] + 100 * map[&'u'] + 10 * map[&'x'] + map[&'a'];
        let a = map[&'a'];
        let slon = 1000 * map[&'s'] + 100 * map[&'l'] + 10 * map[&'o'] + map[&'n'];

        if muxa * a == slon {
            println!(
                "  {}{}{}{}\n×      {}\n--------\n  {}{}{}{}\n",
                map[&'m'], map[&'u'], map[&'x'], map[&'a'],
                a,
                map[&'s'], map[&'l'], map[&'o'], map[&'n'],
            );
            count += 1;
        }
    }

    println!("Знайдено рішень: {}", count);
}
