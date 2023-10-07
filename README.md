# Инструкция к использованию модулей

> [Модуль bubble_sort.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/bubble_sort.rs)

>> 1. Подключение:
>>> ```rust
      mod bubble_sort;
      use crate::bubble_sort::{bubble_sort};
  ```
---
>> 2. Пример использования:
```rust
      fn main() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        println!("Исходный массив: {:?}", arr);
        
        bubble_sort(&mut arr); // Замените на массив, который хотите отсортировать
          
        println!("Отсортированный массив: {:?}", arr);
      }
```

## [Модуль factorial.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/factorial.rs)

1. Подключение:
```rust
      mod factorial;
      use crate::factorial::{factorial};
```
---
2. Пример использования:
```rust
      fn main() {
        let n = 5; // Замените на число, для которого хотите вычислить факториал
        let result = factorial(n);
        println!("Факториал числа {} равен: {}", n, result);
      }
```

## [Модуль fibonacci.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/fibonacci.rs)

1. Подключение:
```rust
      mod fibonacci;
      use crate::fibonacci::{fibonacci};
```
---
2. Пример использования:
```rust
      fn main() {
          let n = 10; // Замените на число, для которого хотите вычислить число Фибоначчи
          let result = fibonacci(n);
          println!("Число Фибоначчи для n={} равно: {}", n, result);
      }
```

## [Модуль singly_linked_list.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/singly_linked_list.rs)

1. Подключение:
```rust
      mod singly_linked_list;
      use crate::singly_linked_list::{LinkedList};
```
---
2. Пример использования:
```rust
      fn main() {
          // Создание и заполнение списка
          let mut list = LinkedList::new();
          list.push(1);
          list.push(2);
          list.push(3);
          list.push(4);
      
          // Вывод исходного списка
          println!("Исходный список:");
          let mut current = &list.head;
          while let Some(node) = current {
              print!("{} -> ", node.data);
              current = &node.next;
          }
          println!("None");
      
          // Разворот списка
          list.reverse();
      
          // Вывод развернутого списка
          println!("Развернутый список:");
          let mut current = &list.head;
          while let Some(node) = current {
              print!("{} -> ", node.data);
              current = &node.next;
          }
          println!("None");
      }
```

