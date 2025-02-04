# Лабораторная работа №1

## Цель работы
Целью данной лабораторной работы является разработка элементарного графического редактора, который реализует построение отрезков с использованием трех различных алгоритмов:
- Алгоритм ЦДА (Цифровой дифференциальный анализатор)
- Целочисленный алгоритм Брезенхема
- Алгоритм Ву

Редактор должен включать в себя панель инструментов, позволяющую выбирать способ генерации отрезков, а также отладочный режим, в котором отображаются шаги построения отрезков на дискретной сетке.

## Задание
Разработать элементарный графический редактор, реализующий построение отрезков с помощью алгоритма ЦДА, целочисленного алгоритма Брезенхема и алгоритма Ву. Вызов способа генерации отрезка задается из пункта меню и доступно через панель инструментов «Отрезки». В редакторе кроме режима генерации отрезков в пользовательском окне должен быть предусмотрен отладочный режим, где отображается пошаговое решение на дискретной сетке.

## Основные теоретические сведения

### Алгоритм ЦДА
ЦДА (Цифровой дифференциальный анализатор) — это метод, используемый для построения отрезков, который основан на вычислении изменений координат по мере продвижения отрезка от начальной точки к конечной. Алгоритм использует простые арифметические операции для вычисления промежуточных точек на сетке, что делает его достаточно быстрым, но менее точным при работе с вертикальными и горизонтальными отрезками.

### Алгоритм Брезенхема
Целочисленный алгоритм Брезенхема — это один из наиболее известных алгоритмов для построения отрезков на экране. Алгоритм используется для построения прямых линий на сетке пикселей, минимизируя количество вычислений и используя только целочисленные операции. Это делает его чрезвычайно быстрым для работы в реальном времени.

### Алгоритм Ву
Алгоритм Ву используется для построения отрезков с антиалиасингом, то есть с плавными переходами цвета. Он основан на вычислениях яркости пикселей и применении этих значений для сглаживания линии, что позволяет сделать линию более плавной и естественной, уменьшив эффект "зубцов", характерных для традиционных методов.

## Листинг кода
### Алгоритм ЦДА
```
pub fn dda_line(start: egui::Pos2, end: egui::Pos2) -> impl Iterator<Item = Vec<(Pixel, Pixel)>> {
    let length = (end.x - start.x).abs().max((end.y - start.y).abs());
    let dx = (end.x - start.x) / length;
    let dy = (end.y - start.y) / length;

    let (x_offset, y_offset) = (start.x.min(end.x), start.y.min(end.y));

    let mut x = start.x + 0.5 * sign(dx);
    let mut y = start.y + 0.5 * sign(dy);

    let mut i = 0.0;
    let first_value = std::iter::once(vec![(
        Pixel::new(x, y, 255),
        Pixel::new((x - x_offset).floor(), (y - y_offset).floor(), 255),
    )]);
    let func_iter = std::iter::from_fn(move || {
        if i <= length {
            let current = Pixel::new(x.floor(), y.floor(), 255);
            let debug = Pixel::new((x - x_offset).floor(), (y - y_offset).floor(), 255);
            x = x + dx;
            y = y + dy;
            i += 1.0;
            Some(vec![(current, debug)])
        } else {
            None
        }
    });

    first_value.chain(func_iter)
}

```

## Вывод
В ходе выполнения лабораторной работы был разработан графический редактор, реализующий построение отрезков с использованием трех алгоритмов: ЦДА, Брезенхема и Ву. Программа предоставляет удобный интерфейс с возможностью выбора алгоритма через меню и панель инструментов.  

Также реализован отладочный режим, позволяющий пошагово отслеживать процесс построения отрезков на дискретной сетке.  

В результате проведенных экспериментов было подтверждено, что:
- Алгоритм ЦДА прост в реализации, но менее точен при работе с крутыми углами.
- Алгоритм Брезенхема эффективен и быстр за счет использования целочисленной арифметики.
- Алгоритм Ву обеспечивает сглаживание линий, улучшая визуальное восприятие.  

Таким образом, работа позволила изучить и сравнить различные методы построения отрезков, их особенности и области применения.


# Лабораторная работа №2

## Цель работы
Целью данной лабораторной работы является разработка элементарного графического редактора, который реализует построение линий второго порядка:
- Окружность
- Эллипс
- Гипербола
- Парабола

Редактор должен включать в себя панель инструментов, позволяющую выбирать тип кривой, а также отладочный режим, в котором отображаются шаги построения на дискретной сетке.

## Задание
Разработать элементарный графический редактор, реализующий построение линий второго порядка: окружность, эллипс, гипербола, парабола. Выбор кривой задается из пункта меню и доступен через панель инструментов «Линии второго порядка». В редакторе кроме режима генерации линий второго порядка в пользовательском окне должен быть предусмотрен отладочный режим, где отображается пошаговое решение на дискретной сетке. 

## Основные теоретические сведения

### Алгоритм построения окружности
Один из наиболее известных алгоритмов построения окружности — это алгоритм Брезенхема. Он позволяет эффективно вычислять пиксели, принадлежащие окружности, используя целочисленные операции. Также можно использовать параметрическое или уравнение окружности в явном виде.

### Алгоритм построения эллипса
Для построения эллипса можно использовать модифицированный алгоритм Брезенхема, который адаптирован для эллиптических форм. Этот алгоритм позволяет минимизировать вычисления и строить эллипс с высокой скоростью.

### Алгоритм построения гиперболы
Гипербола может быть построена с использованием пошагового алгоритма, основанного на ее каноническом уравнении. Чаще всего используется метод на основе дифференциальных уравнений или дискретных итераций.

### Алгоритм построения параболы
Параболу можно построить с использованием пошагового метода, основанного на квадратном уравнении. Применяются аналогичные методы, как и для гиперболы, с учетом особенностей кривизны.

## Листинг кода
### Алгоритм ЦДА
```
pub fn paint_circle(
    start: egui::Pos2,
    end: egui::Pos2,
) -> impl Iterator<Item = Vec<(Pixel, Pixel)>> {
    let mut x = 0;
    let r = start.distance(end) as i32;
    let mut y = r;
    let mut d = 2 - 2 * r;

    let (x_offset, y_offset) = (start.x.min(end.x), start.y.min(end.y));

    let func_iter = std::iter::from_fn(move || {
        if x <= y {
            let (old_x, old_y) = (x, y);
            x += 1;

            if d > 0 {
                y -= 1;
                d += 4 * (x - y) + 10;
            } else {
                d += 4 * x + 6;
            }
            Some(circle_sym(
                start,
                old_x,
                old_y,
                x_offset as i32,
                y_offset as i32,
                r,
            ))
        } else {
            None
        }
    });
    Box::new(func_iter)
}

```

## Вывод
В ходе выполнения лабораторной работы был разработан графический редактор, реализующий построение линий второго порядка: окружности, эллипса, гиперболы и параболы. Программа предоставляет удобный интерфейс с возможностью выбора типа кривой через меню и панель инструментов.  

Также реализован отладочный режим, позволяющий пошагово отслеживать процесс построения на дискретной сетке.  

В результате проведенных экспериментов было подтверждено, что:
- Алгоритм Брезенхема эффективен для построения окружностей и эллипсов.
- Гиперболы и параболы требуют более сложных вычислений и могут быть реализованы разными методами в зависимости от требований к точности и скорости.  

Таким образом, работа позволила изучить и сравнить различные методы построения линий второго порядка, их особенности и области применения.
