# RVLF (Reader Vector Lines from File)

Библиотека написанная на rust использующая SmallVector для уменьшения алокаций
Позволяет быстро загружать векторы строк не загружая все содержимое файла сразу

# Установка

Укажите библиотеку в `Cargo.toml`:
```toml
[dependencies]
reader_vlf = { git = "https://github.com/BigBrainsClub/RVLF" }
```

### **Пример использования**
```rust

use reader_vlf::Reader;

fn main() -> std::io::Result<()> {
    let reader = Reader::new("example.txt")?;
    for (lines, len) in reader {
        println!("{:?}", lines);
        println!("{}", len);
    }
    Ok(())
}
```

## Плюсы
1) Минимальное колличество зависимостей
2) Полностью отсутствует unsafe код
3) Скорость

## Лицензия
Эта библиотека распространяется под лицензией BSD 2-Clause. См. [LICENSE](LICENSE) для деталей.

## Авторы и вклад
 - [@username](https://github.com/BigBrainsClub) - автор и разработчик
 - PR приветствуется! (Буду рад критике и улучшению данной библиотеки)