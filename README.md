# Инструкция к использованию модулей

## [Модуль bubble_sort.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/bubble_sort.rs)
> Подключение:
> > ```rust
> >      mod bubble_sort;
> >      use crate::bubble_sort::{bubble_sort};
> >  ```
***
> Пример использования:
> > ```rust
> >   fn main() {
> >       let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
> >       println!("Исходный массив: {:?}", arr);
> >         
> >       bubble_sort(&mut arr); // Замените на массив, который хотите отсортировать
> >           
> >       println!("Отсортированный массив: {:?}", arr);
> >   }
> > ```

## [Модуль factorial.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/factorial.rs)

> Подключение:
> > ```rust
> >       mod factorial;
> >       use crate::factorial::{factorial};
> >```
***
> Пример использования:
> > ```rust
> >      fn main() {
> >          let n = 5; // Замените на число, для которого хотите вычислить факториал
> >          let result = factorial(n);
> >          println!("Факториал числа {} равен: {}", n, result);
> >      }
> > ```

## [Модуль fibonacci.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/fibonacci.rs)

> Подключение:
> > ```rust
> >      mod fibonacci;
> >      use crate::fibonacci::{fibonacci};
> > ```
***
> Пример использования:
> > ```rust
> >      fn main() {
> >          let n = 10; // Замените на число, для которого хотите вычислить число Фибоначчи
> >          let result = fibonacci(n);
> >          println!("Число Фибоначчи для n={} равно: {}", n, result);
> >      }
> > ```

## [Модуль levenshtein_distance.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/levenshtein_distance.rs)

> Подключение:
> > ```rust
> >      mod levenshtein_distance;
> >      use crate::levenshtein_distance::{levenshtein_distance, similarity_percentage};
> > ```
***
> Пример использования:
> > ```rust
> >   fn main() {
> >      let str1 = "kitten";
> >      let str2 = "sitting";
> >
> >      let distance = levenshtein_distance(str1, str2);
> >      let similarity = similarity_percentage(str1, str2);
> >    
> >      println!(
> >          "Расстояние Левенштейна между '{}' и '{}' равно: {}",
> >          str1, str2, distance
> >      );
> >    
> >      println!(
> >          "Процент совпадения между '{}' и '{}' равен: {:.2}%",
> >          str1, str2, similarity
> >      );
> >   }
> > ```
## [Модуль singly_linked_list.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/singly_linked_list.rs)

> Подключение:
> > ```rust
> >      mod singly_linked_list;
> >      use crate::singly_linked_list::{LinkedList};
> > ```
***
> Пример использования:
> > ```rust
> >      fn main() {
> >          // Создание и заполнение списка
> >          let mut list = LinkedList::new();
> >          list.push(1);
> >          list.push(2);
> >          list.push(3);
> >          list.push(4);
> >      
> >          // Вывод исходного списка
> >          println!("Исходный список:");
> >          let mut current = &list.head;
> >          while let Some(node) = current {
> >              print!("{} -> ", node.data);
> >              current = &node.next;
> >          }
> >          println!("None");
> >      
> >          // Разворот списка
> >          list.reverse();
> >      
> >          // Вывод развернутого списка
> >          println!("Развернутый список:");
> >          let mut current = &list.head;
> >          while let Some(node) = current {
> >              print!("{} -> ", node.data);
> >              current = &node.next;
> >          }
> >          println!("None");
> >      }
> > ```

