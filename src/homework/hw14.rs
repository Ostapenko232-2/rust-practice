use std::collections::HashSet;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point,
    b: Point,
}

// --- Завдання 1: Площа, зайнята прямокутниками ---
fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut covered = HashSet::new();

    for rect in xs {
        let x1 = rect.a.x.min(rect.b.x);
        let x2 = rect.a.x.max(rect.b.x);
        let y1 = rect.a.y.min(rect.b.y);
        let y2 = rect.a.y.max(rect.b.y);

        for x in x1..x2 {
            for y in y1..y2 {
                covered.insert((x, y));
            }
        }
    }

    covered.len() as i32
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

#[test]
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

// --- Завдання 2: Генерація Gray-кодів ---
fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec![String::new()];
    }

    let prev = gray(n - 1);
    let mut result = Vec::new();

    for code in &prev {
        result.push(format!("0{}", code));
    }
    for code in prev.iter().rev() {
        result.push(format!("1{}", code));
    }

    result
}

#[test]
fn gray_test() {
    let test_data = [
        (0, vec![""]),
        (1, vec!["0", "1"]),
        (2, vec!["00", "01", "11", "10"]),
        (3, vec![
            "000", "001", "011", "010",
            "110", "111", "101", "100",
        ]),
        (4, vec![
            "0000", "0001", "0011", "0010",
            "0110", "0111", "0101", "0100",
            "1100", "1101", "1111", "1110",
            "1010", "1011", "1001", "1000",
        ]),
    ];

    for (n, expected) in test_data {
        let result = gray(n);
        assert_eq!(result, expected);
    }
}
