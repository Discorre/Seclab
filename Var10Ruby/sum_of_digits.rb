# Функция для вычисления суммы цифр числа
def sum_of_digits(num)
  sum = 0  # Инициализация переменной для хранения суммы цифр
  # Если число отрицательное, меняем знак
  num = -num if num < 0  # Изменение знака числа на положительный
  while num != 0  # Пока число не равно нулю
    sum += num % 10  # Добавляем последнюю цифру числа к сумме
    num /= 10  # Удаляем последнюю цифру числа
  end
  sum  # Возвращаем сумму цифр числа
end

def main()
  print "Enter the number of numbers: "  # Вывод приглашения для ввода количества чисел
  n = gets.chomp.to_i  # Считываем количество чисел

  print "Enter numbers: "  # Вывод приглашения для ввода чисел
  numbers = gets.chomp.split.map(&:to_i)  # Считываем числа, разделенные пробелами, и преобразуем в массив целых чисел

  numbers.each do |number|  # Перебор каждого числа в массиве
    # Проверяем, что сумма цифр числа состоит из одной цифры
    if sum_of_digits(number) < 10 && number > -10
      print "#{number} "  # Вывод числа, удовлетворяющего условию
    end
  end
  puts  # Переход на новую строку
end

main()  # Вызов главной функции программы