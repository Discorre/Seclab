import Foundation

// Функция для проверки движения фигуры на шахматной доске
func checkMovement(N: Int, M: Int, s: String) -> String {
    var x = 0 // начальная позиция по горизонтали
    var y = 0 // начальная позиция по вертикали

    // Перебор всех ходов в последовательности
    for move in s {
        switch move {
        case "L":
            if x < N/2 && x > -(N/2) {
                x -= 1
            } else {
                return "No" // выход за пределы доски
            }
        case "R":
            if x < N/2 && x > -(N/2) {
                x += 1
            } else {
                return "No" // выход за пределы доски
            }
        case "U":
            if y < M/2 && y > -(M/2) {
                y += 1
            } else {
                return "No" // выход за пределы доски
            }
        case "D":
            if y < M/2 && y > -(M/2) {
                y -= 1
            } else {
                return "No" // выход за пределы доски
            }
        default:
            return "Invalid Command" // некорректная команда
        }
    }

    // Возврат конечной позиции фигуры
    return "(\(x),\(y))"
}

// Пример использования функции
print("Enter N and M: ")
if let input = readLine(), let N1 = Int(input.components(separatedBy: " ")[0]), let M1 = Int(input.components(separatedBy: " ")[1]) {
    print("Enter the sequence of moves: ")
    if let s1 = readLine() {
        print(checkMovement(N: N1, M: M1, s: s1)) // Ожидаемый вывод: No
    }
}
