# Rust_Algorithms_Modules - Это репозиторий с алгоритмами на ЯП Rust
---
- Модуль bubble_sort
  - 1. Подключение:
    ```rust
        mod bubble_sort;
        use crate::bubble_sort::{bubble_sort};
    ```
---
  - 2. Пример использования:
     ```rust
        fn main() {
          let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
          println!("Исходный массив: {:?}", arr);
          
          bubble_sort(&mut arr);
            
          println!("Отсортированный массив: {:?}", arr);
        }
      ```
---
