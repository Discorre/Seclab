class ChessboardMovement
{
    // Функция для проверки движения фигуры на шахматной доске
    static string CheckMovement(int N, int M, string s)
    {
        int x = 0; // начальная позиция по горизонтали
        int y = 0; // начальная позиция по вертикали

        // Перебор всех ходов в последовательности
        foreach (char move in s)
        {
            // Проверка возможности совершить ход и обновление позиции фигуры
            switch (move)
            {
                case 'L':
                    if (x < N / 2 && x > -(N / 2))
                        x--;
                    else
                        return "No"; // выход за пределы доски
                    break;

                case 'R':
                    if (x < N / 2 && x > -(N / 2))
                        x++;
                    else
                        return "No"; // выход за пределы доски
                    break;

                case 'U':
                    if (y < M / 2 && y > -(M / 2))
                        y++;
                    else
                        return "No"; // выход за пределы доски
                    break;

                case 'D':
                    if (y < M / 2 && y > -(M / 2))
                        y--;
                    else
                        return "No"; // выход за пределы доски
                    break;

                default:
                    return "Invalid Command"; // некорректная команда
            }
        }

        // Возврат конечной позиции фигуры
        return $"({x},{y})";
    }

    // Главный метод программы
    static void Main(string[] args)
    {
        // Пример использования функции
        Console.Write("Введите N и M: ");
        string[] input = Console.ReadLine().Split(' ');
        int N1 = int.Parse(input[0]);
        int M1 = int.Parse(input[1]);
        Console.Write("Введите последовательность ходов: ");
        string s1 = Console.ReadLine();
        Console.WriteLine(CheckMovement(N1, M1, s1)); // Ожидаемый вывод: No
    }
}
