mod SumOfDigits;

use std::io;

fn check_movement(N: i32, M: i32, s: &str) -> String {
    let mut x = 0; // начальная позиция по горизонтали
    let mut y = 0; // начальная позиция по вертикали

    for mov in s.chars() {
        match mov {
            'L' => {
                if x < N / 2 && x > -(N / 2) {
                    x -= 1;
                } else {
                    return String::from("No"); // выход за пределы доски
                }
            }
            'R' => {
                if x < N / 2 && x > -(N / 2) {
                    x += 1;
                } else {
                    return String::from("No"); // выход за пределы доски
                }
            }
            'U' => {
                if y < M / 2 && y > -(M / 2) {
                    y += 1;
                } else {
                    return String::from("No"); // выход за пределы доски
                }
            }
            'D' => {
                if y < M / 2 && y > -(M / 2) {
                    y -= 1;
                } else {
                    return String::from("No"); // выход за пределы доски
                }
            }
            _ => return String::from("Invalid Command"), // некорректная команда
        }
    }

    format!("({},{}))", x, y)
}

fn main() {
    // Пример использования функции
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let N1: i32 = iter.next().unwrap().parse().expect("Not an integer");
    let M1: i32 = iter.next().unwrap().parse().expect("Not an integer");
    let s1 = iter.next().unwrap();

    println!("{}", check_movement(N1, M1, s1)); // Ожидаемый вывод: No
}
