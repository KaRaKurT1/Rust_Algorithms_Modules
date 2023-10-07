# Инструкция к использованию модулей

## [Модуль bubble_sort.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/bubble_sort.rs)
> Подключение:
> > ```rust
> >   mod bubble_sort;
> >   use crate::bubble_sort::{bubble_sort};
> >  ```
***
> Пример использования:
> > ```rust
> >   fn main() {
> >     let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
> >     println!("Исходный массив: {:?}", arr);
> >         
> >     bubble_sort(&mut arr); // Замените на массив, который хотите отсортировать
> >           
> >     println!("Отсортированный массив: {:?}", arr);
> >   }
> > ```

## [Модуль factorial.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/factorial.rs)

> Подключение:
> > ```rust
> >   mod factorial;
> >   use crate::factorial::{factorial};
> >```
***
> Пример использования:
> > ```rust
> >   fn main() {
> >     let n = 5; // Замените на число, для которого хотите вычислить факториал
> >     let result = factorial(n);
> >     println!("Факториал числа {} равен: {}", n, result);
> >   }
> > ```

## [Модуль fibonacci.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/fibonacci.rs)

> Подключение:
> > ```rust
> >   mod fibonacci;
> >   use crate::fibonacci::{fibonacci};
> > ```
***
> Пример использования:
> > ```rust
> >   fn main() {
> >     let n = 10; // Замените на число, для которого хотите вычислить число Фибоначчи
> >     let result = fibonacci(n);
> >     println!("Число Фибоначчи для n={} равно: {}", n, result);
> >   }
> > ```

## [Модуль levenshtein_distance.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/levenshtein_distance.rs)

> Подключение:
> > ```rust
> >   mod levenshtein_distance;
> >   use crate::levenshtein_distance::{levenshtein_distance, similarity_percentage};
> > ```
***
> Пример использования:
> > ```rust
> >   fn main() {
> >     let str1 = "kitten";
> >     let str2 = "sitting";
> >
> >     let distance = levenshtein_distance(str1, str2);
> >     let similarity = similarity_percentage(str1, str2);
> >    
> >     println!(
> >       "Расстояние Левенштейна между '{}' и '{}' равно: {}",
> >         str1, str2, distance
> >     );
> >    
> >      println!(
> >        "Процент совпадения между '{}' и '{}' равен: {:.2}%",
> >          str1, str2, similarity
> >      );
> >   }
> > ```
## [Модуль singly_linked_list.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/singly_linked_list.rs)

> Подключение:
> > ```rust
> >   mod singly_linked_list;
> >   use crate::singly_linked_list::{LinkedList};
> > ```
***
> Пример использования:
> > ```rust
> >   fn main() {
> >     // Создание и заполнение списка
> >     let mut list = LinkedList::new();
> >     list.push(1);
> >     list.push(2);
> >     list.push(3);
> >     list.push(4);
> >      
> >     // Вывод исходного списка
> >     println!("Исходный список:");
> >     let mut current = &list.head;
> >     while let Some(node) = current {
> >       print!("{} -> ", node.data);
> >       current = &node.next;
> >     }
> >     println!("None");
> >      
> >     // Разворот списка
> >     list.reverse();
> >      
> >     // Вывод развернутого списка
> >     println!("Развернутый список:");
> >     let mut current = &list.head;
> >     while let Some(node) = current {
> >       print!("{} -> ", node.data);
> >       current = &node.next;
> >     }
> >     println!("None");
> >   }
> > ```

## [Модуль sequential_search.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/sequential_search.rs)

> Подключение:
> > ```rust
> >   mod sequential_search;
> >   use crate::sequential_search::{linear_search};
> > ```
***
> Пример использования:
> > ```rust
> >  fn main() {
> >    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
> >    let target = &5; // Замените на элемент, который хотите найти
> >  
> >    match linear_search(&arr, target) {
> >      Some(index) => println!("Элемент {} найден на позиции {}.", target, index),
> >      None => println!("Элемент {} не найден в массиве.", target),
> >    }
> >  }
> > ```

## [Модуль index_sequential_search.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/index_sequential_search.rs)

> Подключение:
> > ```rust
> >   mod index_sequential_search;
> >   use crate::index_sequential_search::{index_sequential_search};
> >   use std::collections::HashMap; // Нужно из-за использования хэш-таблицы
> > ```
***
> Пример использования:
> > ```rust
> >  fn main() {
> >    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
> >    let mut index: HashMap<i32, usize> = HashMap::new();
> > 
> >    // Создаем индекс
> >    for (i, &value) in data.iter().enumerate() {
> >      index.insert(value, i);
> >    }
> >
> >    let target = &5; // Замените на элемент, который хотите найти
> >
> >    match index_sequential_search(&data, &index, target) {
> >      Some(index) => println!("Элемент {} найден на позиции {}.", target, index),
> >      None => println!("Элемент {} не найден в массиве.", target),
> >    }
> >  }
> > ```

## [Модуль binary_search.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/binary_search.rs)

> Подключение:
> > ```rust
> >   mod binary_search;
> >   use crate::binary_search::{binary_search}; 
> > ```
***
> Пример использования:
> > ```rust
> >  fn main() {
> >    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
> >    let target = &5; // Замените на элемент, который хотите найти
> >  
> >    match binary_search(&arr, target) {
> >      Some(index) => println!("Элемент {} найден на позиции {}.", target, index),
> >      None => println!("Элемент {} не найден в массиве.", target),
> >    }
> >  }
> > ```  

## [Модуль insertion_sort.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/insertion_sort.rs)
> Подключение:
> > ```rust
> >   mod insertion_sort;
> >   use crate::insertion_sort::{insertion_sort};
> >  ```
***
> Пример использования:
> > ```rust
> >   fn main() {
> >     let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
> >     println!("Исходный массив: {:?}", arr);
> >         
> >     insertion_sort(&mut arr); // Замените на массив, который хотите отсортировать
> >           
> >     println!("Отсортированный массив: {:?}", arr);
> >   }
> > ```

## [Модуль selection_sort.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/selection_sort.rs)
> Подключение:
> > ```rust
> >   mod selection_sort;
> >   use crate::selection_sort::{selection_sort};
> >  ```
***
> Пример использования:
> > ```rust
> >   fn main() {
> >     let mut arr = vec![28, 422, 15, 66, 72, 19, 880];
> >     println!("Исходный массив: {:?}", arr);
> >         
> >     selection_sort(&mut arr); // Замените на массив, который хотите отсортировать
> >           
> >     println!("Отсортированный массив: {:?}", arr);
> >   }
> > ```

## [Модуль cocktail_sort.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/cocktail_sort.rs)
> Подключение:
> > ```rust
> >   mod cocktail_sort;
> >   use crate::cocktail_sort::{cocktail_sort};
> >  ```
***
> Пример использования:
> > ```rust
> >   fn main() {
> >     let mut arr = vec![4, 3, 2, 1, 5, 666, 818];
> >     println!("Исходный массив: {:?}", arr);
> >         
> >     cocktail_sort(&mut arr); // Замените на массив, который хотите отсортировать
> >           
> >     println!("Отсортированный массив: {:?}", arr);
> >   }
> > ```

## [Модуль shell_sort.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/shell_sort.rs)
> Подключение:
> > ```rust
> >   mod shell_sort;
> >   use crate::shell_sort::{shell_sort};
> >  ```
***
> Пример использования:
> > ```rust
> >   fn main() {
> >     let mut arr = vec![999, 374, 215, 1211, 222, 3, 2];
> >     println!("Исходный массив: {:?}", arr);
> >         
> >     shell_sort(&mut arr); // Замените на массив, который хотите отсортировать
> >           
> >     println!("Отсортированный массив: {:?}", arr);
> >   }
> > ```

## [Модуль tree_sort.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/tree_sort.rs)
> Подключение:
> > ```rust
> >   mod tree_sort;
> >   use crate::tree_sort::{tree_sort};
> >  ```
***
> Пример использования:
> > ```rust
> >   fn main() {
> >     let arr = vec![22, 56, 95, 81, 248, 888, 120];
> >     println!("Исходный массив: {:?}", arr);
> >         
> >     let sorted_arr = tree_sort(&arr);
> >           
> >     println!("Отсортированный массив: {:?}", sorted_arr);
> >   }
> > ```

## [Модуль heap_sort.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/heap_sort.rs)
> Подключение:
> > ```rust
> >   mod heap_sort;
> >   use crate::heap_sort::{heap_sort};
> >  ```
***
> Пример использования:
> > ```rust
> >   fn main() {
> >     let mut arr = vec![111, 4678, 1255, 12462, 2222, 1133, 90];
> >     println!("Исходный массив: {:?}", arr);
> >         
> >     heap_sort(&mut arr); // Замените на массив, который хотите отсортировать
> >           
> >     println!("Отсортированный массив: {:?}", arr);
> >   }
> > ```

## [Модуль quick_sort.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/quick_sort.rs)
> Подключение:
> > ```rust
> >   mod quick_sort;
> >   use crate::quick_sort::{quick_sort};
> >  ```
***
> Пример использования:
> > ```rust
> >   fn main() {
> >     let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
> >     println!("Исходный массив: {:?}", arr);
> >         
> >     quick_sort(&mut arr); // Замените на массив, который хотите отсортировать
> >           
> >     println!("Отсортированный массив: {:?}", arr);
> >   }
> > ```

## [Модуль merge_sort.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/merge_sort.rs)
> Подключение:
> > ```rust
> >   mod merge_sort;
> >   use crate::merge_sort::{merge_sort};
> >  ```
***
> Пример использования:
> > ```rust
> >   fn main() {
> >     let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
> >     println!("Исходный массив: {:?}", arr);
> >         
> >     merge_sort(&mut arr); // Замените на массив, который хотите отсортировать
> >           
> >     println!("Отсортированный массив: {:?}", arr);
> >   }
> > ```

## [Модуль pre_order.rs](https://github.com/KaRaKurT1/Rust_Algorithms_Modules/blob/main/pre_order.rs)
> Подключение:
> > ```rust
> >   mod pre_order;
> >   use crate::pre_order::{TreeNode, pre_order_traversal};
> >  ```
***
> Пример использования:
> > ```rust
> >   fn main() {
> >     // Создаем бинарное дерево
> >     let mut root = TreeNode::new("F");
> >     let mut node2 = TreeNode::new("B");
> >     let node3 = TreeNode::new("A");
> >     let mut node4 = TreeNode::new("D");
> >     let node5 = TreeNode::new("C");
> >     let node6 = TreeNode::new("E");
> >     let mut node7 = TreeNode::new("G");
> >     let node8 = TreeNode::new("H");
> >     let mut node9 = TreeNode::new("I");
> > 
> >     node9.left = Some(Box::new(node8));
> >     node7.right = Some(Box::new(node9));
> >     root.right = Some(Box::new(node7));
> >     node4.right = Some(Box::new(node6));
> >     node4.left = Some(Box::new(node5));
> >     node2.right = Some(Box::new(node4));
> >     node2.left = Some(Box::new(node3));
> >     root.left = Some(Box::new(node2));
> >
> >     let mut result = Vec::new();
> >     pre_order_traversal(&Some(Box::new(root)), &mut result);
> >
> >     println!("Результат обхода в порядке pre-order: {:?}", result);
> >   }
> > ```
