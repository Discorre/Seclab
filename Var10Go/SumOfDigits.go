package main

import (
	"fmt"
)

// Функция для вычисления суммы цифр числа
func sumOfDigits(num int) int {
	sum := 0 // Инициализируем переменную для хранения суммы цифр числа
	for num != 0 { // Пока число не равно 0
		sum += num % 10 // Добавляем к сумме последнюю цифру числа
		num /= 10 // Удаляем последнюю цифру числа
	}
	return sum // Возвращаем сумму цифр числа
}

func main() {
	var n int // Объявляем переменную для хранения количества чисел
	fmt.Print("Enter the number of numbers: ") // Выводим приглашение для ввода количества чисел
	fmt.Scan(&n) // Считываем количество чисел

	numbers := make([]int, n) // Создаем слайс для хранения чисел
	fmt.Print("Enter numbers: ") // Выводим приглашение для ввода чисел
	for i := 0; i < n; i++ { // Итерируемся от 0 до n-1
		fmt.Scan(&numbers[i]) // Считываем числа и добавляем их в слайс
	}

	fmt.Print("Numbers whose sum of digits consists of one decimal place: ") // Выводим заголовок для вывода чисел
	for _, num := range numbers { // Итерируемся по числам из слайса
		if sumOfDigits(num) < 10 && num > -10 { // Проверяем, что сумма цифр числа состоит из одной цифры и число не больше -10
			fmt.Printf("%d ", num) // Выводим число
		}
	}
	fmt.Println() // Переходим на новую строку
}
