# Rust_Algorithms_Modules - Это репозиторий с алгоритмами на ЯП Rust

## [Модуль bubble_sort.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/bubble_sort.rs)

1. Подключение:
```rust
      mod bubble_sort;
      use crate::bubble_sort::{bubble_sort};
  ```
---
2. Пример использования:
```rust
      fn main() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        println!("Исходный массив: {:?}", arr);
        
        bubble_sort(&mut arr);
          
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

