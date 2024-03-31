use std::io; // Импорт стандартной библиотеки ввода/вывода

// Функция для вычисления суммы цифр числа
fn sum_of_digits(mut num: i32) -> i32 {
    let mut sum = 0; // Инициализация переменной для хранения суммы цифр числа
    // Если число отрицательное, меняем знак
    if num < 0 {
        num *= -1;
    }
    while num != 0 { // Пока число не равно нулю
        sum += num % 10; // Добавляем последнюю цифру числа к сумме
        num /= 10; // Удаляем последнюю цифру числа
    }
    sum // Возвращаем сумму цифр числа
}

fn main() {
    println!("Enter the number of numbers:"); // Вывод сообщения для ввода количества чисел
    let mut input = String::new(); // Создание переменной для хранения ввода
    io::stdin().read_line(&mut input).expect("Failed to read line"); // Считывание строки из стандартного ввода и запись в переменную input

    let n: i32 = input.trim().parse().expect("Please enter a valid number"); // Обрезка лишних символов и преобразование строки в число типа i32

    println!("Enter numbers separated by space:"); // Вывод сообщения для ввода чисел через пробел
    input.clear(); // Очистка переменной input
    io::stdin().read_line(&mut input).expect("Failed to read line"); // Считывание строки из стандартного ввода и запись в переменную input

    let numbers: Vec<i32> = input
        .split_whitespace() // Разделение строки на отдельные числа по пробелам
        .map(|s| s.parse().expect("Please enter valid numbers")) // Преобразование каждой подстроки в число типа i32
        .collect(); // Сбор чисел в вектор

    for number in numbers.iter() { // Перебор каждого числа в векторе
        // Проверяем, что сумма цифр числа состоит из одной цифры
        if sum_of_digits(*number) < 10 && *number > -10 {
            print!("{} ", number); // Вывод числа, удовлетворяющего условию
        }
    }
    println!(); // Переход на новую строку
}
