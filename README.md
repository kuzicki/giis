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
```rust
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
### Алгоритм отрисовки окружности
```rust
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

# Лабораторная работа №3

## Цель работы
Целью данной лабораторной работы является изучение и реализация построения кривых с использованием различных методов, включая кривые Безье, B-сплайны и сплайны Эрмита.

В рамках работы необходимо:
- Разобраться с теоретическими основами построения кривых Безье, B-сплайнов и сплайнов Эрмита.
- Реализовать построение данных кривых на основе матричных представлений.
- Исследовать их свойства и особенности.

## Задание
Реализовать построение кривых с использованием различных методов:
- Кривые Безье.
- B-сплайны.
- Сплайны Эрмита.

Построение кривых должно быть реализовано в виде матричных представлений.
Для каждого метода необходимо визуализировать построенные кривые и продемонстрировать их свойства.
## Основные теоретические сведения

### Форма Эрмита

Форма Эрмита определяется контрольными точками и их производными (касательными). Это позволяет явно управлять направлением кривой в каждой точке.

Кубическая кривая Эрмита описывается уравнением:

$$
P(t) = H_0(t) P_0 + H_1(t) P_1 + H_2(t) T_0 + H_3(t) T_1,
$$

где $P_0$ и $P_1$ - опорные точки, $T_0$ и $T_1$ - касательные в этих точках, а $H_i(t)$ - базисные функции Эрмита:

$$
H_0(t) = 2t^3 - 3t^2 + 1,
$$

$$
H_1(t) = -2t^3 + 3t^2,
$$

$$
H_2(t) = t^3 - 2t^2 + t,
$$

$$
H_3(t) = t^3 - t^2.
$$

#### Матрица Эрмита

В матричной форме кривая Эрмита представляется так:

$$
\begin{bmatrix} x(t) \\ y(t) \\ z(t) \end{bmatrix} =
\begin{bmatrix} t^3 & t^2 & t & 1 \end{bmatrix}
\begin{bmatrix}
2 & -2 & 1 & 1 \\
-3 & 3 & -2 & -1 \\
0 & 0 & 1 & 0 \\
1 & 0 & 0 & 0
\end{bmatrix}
\begin{bmatrix} P_0 \\ P_1 \\ T_0 \\ T_1 \end{bmatrix}.
$$

### Кривые Безье

Кривые Безье используются для моделирования гладких кривых и широко применяются в компьютерной графике и анимации. Они определяются параметрическими уравнениями вида:

$$
P(t) = \sum_{i=0}^{n} B_i^n (t) P_i
$$

где $P_i$ - контрольные точки, а $B_i^n(t)$ - многочлены Бернштейна:

$$
B_i^n (t) = \binom{n}{i} t^i (1-t)^{n-i}, \quad 0 \leq t \leq 1.
$$



#### Матрица Безье

Для кубического случая ($n = 3$) можно представить кривую в матричной форме:

$$
\begin{bmatrix} x(t) \\ y(t) \\ z(t) \end{bmatrix} =
\begin{bmatrix} t^3 & t^2 & t & 1 \end{bmatrix}
\begin{bmatrix}
-1 & 3 & -3 & 1 \\
3 & -6 & 3 & 0 \\
-3 & 3 & 0 & 0 \\
1 & 0 & 0 & 0
\end{bmatrix}
\begin{bmatrix} P_0 \\ P_1 \\ P_2 \\ P_3 \end{bmatrix}.
$$

### B-сплайны

B-сплайны (Basis splines) — это обобщение кривых Безье, позволяющее строить гладкие кривые с помощью кусочно-полиномиальных функций.

Формула B-сплайнов:

$$
P(t) = \sum_{i=0}^{n} N_{i,k}(t) P_i,
$$

где $N_{i,k}(t)$ - базисные функции сплайна степени $k$, определяемые рекурсивно:

$$
N_{i,1}(t) = \begin{cases} 1, & t_i \leq t < t_{i+1} \\ 0, & \text{иначе} \end{cases},
$$

$$
N_{i,k}(t) = \frac{t - t_i}{t_{i+k-1} - t_i} N_{i,k-1}(t) + \frac{t_{i+k} - t}{t_{i+k} - t_{i+1}} N_{i+1,k-1}(t).
$$

#### Матрица B-сплайна (кубический случай)

Матрица кубического B-сплайна:

$$
\begin{bmatrix} x(t) \\ y(t) \\ z(t) \end{bmatrix} =
\begin{bmatrix} t^3 & t^2 & t & 1 \end{bmatrix}
\frac{1}{6} \begin{bmatrix}
-1 & 3 & -3 & 1 \\
3 & -6 & 3 & 0 \\
-3 & 0 & 3 & 0 \\
1 & 4 & 1 & 0
\end{bmatrix}
\begin{bmatrix} P_0 \\ P_1 \\ P_2 \\ P_3 \end{bmatrix}.
$$

## Листинг кода
### Алгоритм отрисовки кривых Эрмита
```rust
pub fn generate_hermite_curve(p0: Pos2, p1: Pos2, p2: Pos2, p3: Pos2, pixels: &mut Vec<Pixel>) {
    let t0 = Vec2::new((p1.x - p0.x) * 3.0, (p1.y - p0.y) * 3.0);
    let t1 = Vec2::new((p3.x - p2.x) * 3.0, (p3.y - p2.y) * 3.0);

    let points = [p0, p1, p2, p3];
    let max_distance = points
        .iter()
        .enumerate()
        .flat_map(|(i, &p1)| points.iter().skip(i + 1).map(move |&p2| p1.distance(p2)))
        .fold(0.0, f32::max);
    let steps = (max_distance * SCALE).max(MIN_SCALE) as usize;
    let hermite_matrix = [
        [2.0, -2.0, 1.0, 1.0],
        [-3.0, 3.0, -2.0, -1.0],
        [0.0, 0.0, 1.0, 0.0],
        [1.0, 0.0, 0.0, 0.0],
    ];

    let x_coeffs = matrix_multiply(&hermite_matrix, &[p0.x, p2.x, t0.x, t1.x]);
    let y_coeffs = matrix_multiply(&hermite_matrix, &[p0.y, p2.y, t0.y, t1.y]);

    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let t_vec = [t * t * t, t * t, t, 1.0];

        let x = multiply_hermite_coeffs(&x_coeffs, &t_vec);
        let y = multiply_hermite_coeffs(&y_coeffs, &t_vec);

        pixels.push(Pixel::new_black(x, y, 255));
    }
}

```

## Вывод
В ходе выполнения лабораторной работы были изучены и реализованы три метода построения кривых: Безье, B-сплайны и сплайны Эрмита. Все методы были представлены в матричной форме, что позволило унифицировать их вычисление.

Результаты исследования показали, что:
- Сплайны Эрмита позволяют точно задать касательные направления в начальных и конечных точках.
- Кривые Безье дают удобное управление формой с помощью контрольных точек.
- B-сплайны обладают большей гибкостью за счет локального контроля формы.

Таким образом, работа позволила изучить принципы построения кривых и сравнить их свойства в различных приложениях.

# Лабораторная работа №4

## Цель работы
Целью данной лабораторной работы является разработка графической программы, выполняющей геометрические преобразования над трехмерным объектом: перемещение, поворот, скалирование, отображение и перспектива. Программа должна предусматривать считывание координат 3D объекта из текстового файла, обработку клавиатуры и выполнение преобразований в зависимости от нажатых клавиш. Все преобразования должны выполняться с использованием матричного аппарата и представления координат в однородных координатах.

## Задание
Разработать программу, которая выполняет геометрические преобразования над трехмерным объектом: перемещение, поворот, скалирование, отображение и перспектива. Программа должна загружать координаты объекта из текстового файла и использовать матричные преобразования для выполнения операций. Важно предусмотреть управление клавишами для выполнения различных операций над объектом.

## Основные теоретические сведения

### Матричные преобразования
Матричные преобразования — это метод, используемый для выполнения различных операций над координатами объектов в компьютерной графике. В частности, с помощью матриц можно выполнять такие преобразования, как поворот, масштабирование, перемещение и проекцию. Все эти операции могут быть объединены в одну большую операцию с помощью умножения матриц. В 3D-графике часто используется представление координат в однородных координатах (гомогенных координатах), что позволяет объединить несколько преобразований в одно.

#### Перемещение
Перемещение объекта в 3D пространстве осуществляется с помощью матрицы трансляции. Трансляция изменяет координаты объекта, сдвигая его на определенную величину в трех осях. Матрица трансляции для перемещения на (Tx, Ty, Tz) выглядит следующим образом:

$$
\begin{bmatrix} x \\ y \\ z \\ 1 \end{bmatrix} =
\begin{bmatrix} 
1 & 0 & 0 & T_x \\ 
0 & 1 & 0 & T_y \\ 
0 & 0 & 1 & T_z \\ 
0 & 0 & 0 & 1 
\end{bmatrix}
\begin{bmatrix} x' \\ y' \\ z' \\ 1 \end{bmatrix}
$$


Каждая вершина объекта умножается на эту матрицу для выполнения перемещения.

#### Поворот
Поворот объекта вокруг одной из осей в 3D пространстве осуществляется с помощью матрицы поворота. Для каждой из трех осей (X, Y, Z) существует своя матрица поворота.

- Поворот вокруг оси X:

$$
R_x(\theta) = \begin{bmatrix}
1 & 0 & 0 & 0 \\
0 & \cos(\theta) & -\sin(\theta) & 0 \\
0 & \sin(\theta) & \cos(\theta) & 0 \\
0 & 0 & 0 & 1
\end{bmatrix}
$$

- Поворот вокруг оси Y:

$$
R_y(\theta) = \begin{bmatrix}
\cos(\theta) & 0 & \sin(\theta) & 0 \\
0 & 1 & 0 & 0 \\
-\sin(\theta) & 0 & \cos(\theta) & 0 \\
0 & 0 & 0 & 1
\end{bmatrix}
$$

- Поворот вокруг оси Z:

$$
R_z(\theta) = \begin{bmatrix}
\cos(\theta) & -\sin(\theta) & 0 & 0 \\
\sin(\theta) & \cos(\theta) & 0 & 0 \\
0 & 0 & 1 & 0 \\
0 & 0 & 0 & 1
\end{bmatrix}
$$

Эти матрицы используются для выполнения поворота объекта в 3D пространстве.

#### Масштабирование
Масштабирование объекта происходит с помощью матрицы масштабирования, которая изменяет размеры объекта по всем осям. Матрица масштабирования для масштабирования на коэффициенты (Sx, Sy, Sz) выглядит следующим образом:

$$
S = \begin{bmatrix}
S_x & 0 & 0 & 0 \\
0 & S_y & 0 & 0 \\
0 & 0 & S_z & 0 \\
0 & 0 & 0 & 1
\end{bmatrix}
$$

Умножив координаты каждой вершины объекта на эту матрицу, можно масштабировать объект в 3D пространстве.

#### Проекция
Проекция — это преобразование, которое отображает 3D объект на двумерную плоскость экрана. Одним из наиболее распространенных типов проекции является перспектива. Проекционные матрицы могут быть орто-программными и перспективными. Перспективная проекция позволяет объектам, находящимся дальше от камеры, казаться меньше, создавая эффект глубины.

Перспективная проекционная матрица выглядит следующим образом:

$$
P = \begin{bmatrix}
\frac{f}{w} & 0 & 0 & 0 \\
0 & \frac{f}{h} & 0 & 0 \\
0 & 0 & \frac{f+z_{far}}{z_{near}-z_{far}} & \frac{2 f z_{near}}{z_{near}-z_{far}} \\
0 & 0 & -1 & 0
\end{bmatrix}
$$

где \( f \) — фокусное расстояние, \( z_near \) и \( z_far \) — расстояния до ближней и дальней плоскости отсечения, \( w \) и \( h \) — ширина и высота экрана.

#### Однородные координаты
Для удобства работы с преобразованиями в 3D графике используются однородные координаты. Это расширение трехмерного пространства на четвертое измерение, которое позволяет объединить все преобразования (перемещение, поворот, масштабирование) в одну матричную операцию. Каждая точка в 3D пространстве представляется как вектор \([x, y, z]\), а в однородных координатах она представляется как вектор \([x, y, z, 1]\).

### Реализация
Программа, разработанная в рамках этой лабораторной работы, реализует указанные геометрические преобразования с использованием матричных операций для выполнения перемещения, поворота, масштабирования, отображения и перспективы. Все операции реализованы через матричные умножения, и результаты применяются к вершинам 3D объекта. Также предусмотрена возможность считывания координат из текстового файла и управление преобразованиями с помощью клавиш.

## Листинг кода
```rust
    fn draw(&self, painter: &eframe::egui::Painter) {
        let transform_matrix = self.get_transform_matrix();

        let perspective_matrix = get_perspective_matrix(
            self.fov,
            self.screen_width / self.screen_height,
            0.1,                                   
            10000000.0,                            
        );

        let transformed_vertices: Vec<Vector3<f32>> = self
            .vertices
            .iter()
            .map(|v| {
                let homogeneous_vertex = transform_matrix * Vector4::new(v.x, v.y, v.z, 1.0);
                let transformed = perspective_matrix * homogeneous_vertex;

                Vector3::new(
                    transformed.x / transformed.w,
                    transformed.y / transformed.w,
                    transformed.z / transformed.w,
                )
            })
            .collect();

        let mut points_2d = Vec::new();
        for vertex in &transformed_vertices {
            let coord = world_to_screen(*vertex, self.screen_width, self.screen_height);
            points_2d.push(coord);
        }

        for chunk in self.indices.chunks(3) {
            let p1 = points_2d[chunk[0] as usize];
            let p2 = points_2d[chunk[1] as usize];
            let p3 = points_2d[chunk[2] as usize];
            let stroke = self.get_outline_color();
            painter.add(Shape::line_segment([p1, p2], stroke));
            painter.add(Shape::line_segment([p2, p3], stroke));
            painter.add(Shape::line_segment([p3, p1], stroke));
        }
    }

```

### Вывод
В ходе выполнения лабораторной работы была разработана графическая программа, которая реализует базовые геометрические преобразования: перемещение, поворот, скалирование и проекцию для трехмерных объектов. Программа позволяет считывать 3D координаты из файла и управлять преобразованиями с помощью клавиш. В результате выполнения программы можно наблюдать, как изменяется объект в зависимости от выбранного преобразования, что позволяет лучше понять работу с матричными преобразованиями в 3D графике.


# Лабораторная работа №5

## Цель работы
Целью данной лабораторной работы является разработка элементарного графического редактора, который реализует построение полигонов и поддерживает следующие функции:

* Проверка полигона на выпуклость.
* Нахождение внутренних нормалей полигона.
* Построение выпуклой оболочки методом обхода Грэхема и методом Джарвиса.
* Определение точек пересечения отрезка со стороной полигона.
* Определение принадлежности введенной точки полигону.
Реализованная программа должна предоставлять возможность выбора метода построения выпуклой оболочки через меню и панель инструментов «Построение полигонов». Также графический редактор должен позволять рисовать линии первого порядка, как в предыдущей лабораторной работе.

## Задание
Разработать элементарный графический редактор, который реализует построение полигонов и выполнения операций с ними, таких как:

* Проверка выпуклости полигона.
* Нахождение внутренних нормалей полигона.
* Построение выпуклой оболочки с помощью методов обхода Грэхема и Джарвиса.
* Определение точек пересечения отрезков со сторонами полигона.
* Определение принадлежности точки полигону.

Выбор метода для построения выпуклой оболочки должен быть доступен через меню и панель инструментов «Построение полигонов». Программа должна поддерживать рисование линий первого порядка, как в лабораторной работе №1, и позволять пользователю вводить точки для рисования полигона и выполнять указанные операции.

## Основные теоретические сведения
### Проверка полигона на выпуклость
Полигон считается выпуклым, если для каждой из его сторон, любая внутренняя точка, которая соединяет два соседних угла, лежит внутри или на границе полигона. Визуально это означает, что все его углы направлены наружу. Для проверки выпуклости используется метод вычисления углов между векторами, образующими стороны полигона.

### Внутренние нормали полигона
Нормали полигона — это векторы, перпендикулярные сторонам, направленные внутрь или наружу от полигона. Внутренние нормали направлены внутрь, что позволяет использовать их для различных геометрических вычислений, например, при проверке точек на принадлежность полигону.

### Метод обхода Грэхема
Метод Грэхема — это алгоритм для поиска выпуклой оболочки множества точек. Алгоритм работает путем сортировки точек по углу относительно начальной точки и их последующего обхода, исключая все точки, которые находятся внутри оболочки.

### Метод Джарвиса (обход Джарвиса)
Метод Джарвиса, также известный как алгоритм "обхода по часовой стрелке", строит выпуклую оболочку, начиная с самой левой точки и последовательно добавляя точки, образующие выпуклый угол. Алгоритм повторяет процесс до тех пор, пока не вернется в исходную точку.

### Точки пересечения отрезков
Пересечение отрезков вычисляется через решения системы уравнений прямых, образующих отрезки. Это позволяет находить точку пересечения двух отрезков.

### Принадлежность точки полигону
Принадлежность точки полигону проверяется путем вычисления направления на каждом из отрезков и проверки, находится ли точка внутри или вне границ полигона.

## Листинг кода
```rust
    fn test_convex(&self) -> bool {
        if self.control_points.len() < 3 {
            return false;
        }

        let mut last_sign: Option<bool> = None;
        let n = self.control_points.len();
        for i in 0..n {
            let o = self.control_points[i];
            let a = self.control_points[(i + 1) % n];
            let b = self.control_points[(i + 2) % n];

            let cp = cross_product(o, a, b);

            if cp != 0.0 {
                let current_sign = cp > 0.0;

                if last_sign.is_none() {
                    last_sign = Some(current_sign);
                } else if last_sign != Some(current_sign) {
                    return false;
                }
            }
        }

        true
    }
```

### Вывод
В ходе выполнения лабораторной работы был разработан графический редактор, реализующий построение полигонов с использованием методов проверки выпуклости, нахождения внутренних нормалей, построения выпуклой оболочки методом обхода Грэхема и методом Джарвиса.

Программа предоставляет удобный интерфейс для работы с полигонами, позволяет пользователю рисовать линии, строить многоугольники, а также выполнять операции по нахождению точек пересечения отрезков и проверке принадлежности точки полигону. Выбор метода для построения выпуклой оболочки доступен через меню и панель инструментов «Построение полигонов».

Результаты показали, что:

* Метод Грэхема является быстрым и эффективным для построения выпуклых оболочек.
* Метод Джарвиса дает более точный результат, но работает медленнее.
* Программа успешно определяет выпуклость полигона и выполняет операции с ним, включая пересечение отрезков и принадлежность точек полигону.

Таким образом, работа позволила изучить и реализовать алгоритмы для работы с многоугольниками и полигональными оболочками, а также практическое применение геометрических методов в графическом редакторе.

# Лабораторная работа №6

## Цель работы
Целью данной лабораторной работы является разработка элементарного графического редактора, реализующего построение полигонов и их заполнение с использованием различных алгоритмов:

* Алгоритм растровой развертки с упорядоченным списком ребер
* Алгоритм растровой развертки с упорядоченным списком ребер, использующий список активных ребер
* Простой алгоритм заполнения с затравкой
* Построчный алгоритм заполнения с затравкой

Редактор должен включать в себя панель инструментов, позволяющую выбирать метод заполнения полигона, а также отладочный режим, в котором отображаются шаги выполнения алгоритмов.

## Задание
Разработать графический редактор, реализующий построение полигонов и их заполнение с использованием указанных алгоритмов. Выбор метода заполнения задается из меню и доступен через панель инструментов «Алгоритмы заполнения полигонов». В редакторе должен быть предусмотрен режим отладки, где пошагово отображается процесс выполнения алгоритма.

## Основные теоретические сведения
### Алгоритм растровой развертки с упорядоченным списком ребер
Этот алгоритм основан на сортировке всех ребер полигона по их начальной координате и последовательном сканировании строк изображения. Для каждой строки вычисляются пересечения ребер с текущей строкой, после чего выполняется заливка соответствующих интервалов.

### Алгоритм растровой развертки с активными ребрами
Этот метод является оптимизированной версией предыдущего. Вместо обработки всех ребер одновременно, используется список активных ребер, который обновляется по мере продвижения вниз по изображению. Это позволяет уменьшить вычислительные затраты и ускорить процесс заливки полигона.

### Простой алгоритм заполнения с затравкой
Данный алгоритм начинает заливку из одной заданной точки (затравки) и рекурсивно распространяет цвет на соседние пиксели, пока не будет достигнута граница полигона. Этот метод прост в реализации, но может требовать значительных вычислительных ресурсов.

### Построчный алгоритм заполнения с затравкой
В отличие от простого затравочного алгоритма, этот метод работает построчно, заполняя полигон горизонтальными линиями. Это значительно снижает затраты на вычисления и уменьшает использование памяти.

## Листинг кода
```rust
    fn third(&mut self) { // Простой алгоритм заполнения с затравкой
        let mut visited = HashSet::new();
        let step = 0.000001;
        let start = find_centroid(&self.control_points);
        let sx = start.x as i32;
        let sy = start.y as i32;
        let mut stack = vec![(sx, sy)];
        let polygon = self.control_points.clone();
        let func_iter = std::iter::from_fn(move || {
            loop {
                if let Some((x, y)) = stack.pop() {
                    if visited.contains(&(x, y)) {
                        continue;
                    }

                    let p = Pos2 {
                        x: x as f32,
                        y: y as f32,
                    };
                    if is_on_boundary(&polygon, p, step / 2.0) || !is_inside(&polygon, p) {
                        continue;
                    }

                    visited.insert((x, y));

                    stack.push((x + 1, y));
                    stack.push((x - 1, y));
                    stack.push((x, y + 1));
                    stack.push((x, y - 1));
                    return Some(vec![get_rect_shape(p.x, p.y)]);
                } else {
                    return None;
                }
            }
        });

        self.update_func = RefCell::new(Box::new(func_iter));
    }
```

### Вывод
В ходе выполнения лабораторной работы был разработан графический редактор, реализующий построение полигонов и их заполнение с использованием различных алгоритмов. Программа предоставляет удобный интерфейс, позволяющий выбирать алгоритм заполнения через меню и панель инструментов.

Также реализован отладочный режим, позволяющий пошагово отслеживать процесс выполнения алгоритмов.

В результате проведенных экспериментов было подтверждено, что:

* Алгоритм растровой развертки с упорядоченным списком ребер прост в реализации, но требует значительных вычислений при обработке сложных полигонов.
* Использование списка активных ребер значительно ускоряет работу алгоритма.
* Затравочные алгоритмы удобны для заполнения сложных областей, но могут потреблять много памяти.
* Построчный алгоритм затравочного заполнения является оптимальным по времени выполнения.

Таким образом, лабораторная работа позволила изучить и сравнить различные методы заполнения полигонов, их особенности и области применения.

# Лабораторная работа №7

## Цель работы
Целью данной лабораторной работы является разработка графической программы, реализующей триангуляцию Делоне и построение диаграммы Вороного по заданному набору точек. Для построения диаграммы Вороного используется предварительно выполненная триангуляция Делоне.

## Задание
Разработать графическую программу, выполняющую:

* Триангуляцию Делоне по заданному набору точек.

* Построение диаграммы Вороного на основе полученной триангуляции.

Выбор метода задается из пункта меню и должен быть доступен через панель инструментов «Геометрические построения». В программе должен быть предусмотрен режим отладки, в котором отображается пошаговый процесс построения триангуляции и диаграммы Вороного.

## Основные теоретические сведения
### Триангуляция Делоне
Триангуляция Делоне — это разбиение множества точек на треугольники таким образом, чтобы выполнялось свойство Делоне: ни одна точка из множества не лежит внутри описанной окружности любого треугольника. Данный метод гарантирует получение триангуляции с хорошими угловыми свойствами и минимизацией длин рёбер.

### Диаграмма Вороного
Диаграмма Вороного — это разбиение плоскости на области, каждая из которых соответствует одной из исходных точек. Любая точка внутри области ближе к своей опорной точке, чем к любой другой. Для построения диаграммы Вороного часто используется триангуляция Делоне, так как рёбра диаграммы Вороного являются биссектрисами рёбер триангуляции Делоне.

## Листинг кода
```rust
    pub fn new(points: Vec<Pos2>) -> Self {
        let mut delone = Self { triangles: vec![] };
        let super_triangle = Triangle {
            a: Pos2 { x: -300.0, y: 0.0 },
            b: Pos2 { x: 1000.0, y: 0.0 },
            c: Pos2 {
                x: 650.0,
                y: 2000.0,
            },
        };
        delone.triangles.push(super_triangle);
        for point in points {
            delone.add_point(point);
        }
        let mut bad_triangles = vec![];
        for t in &delone.triangles {
            'outer: for super_edges in &super_triangle.get_edges() {
                for edges in t.get_edges() {
                    if super_edges.a == edges.a
                        || super_edges.a == edges.b
                        || super_edges.b == edges.a
                        || super_edges.b == edges.b
                    {
                        bad_triangles.push(*t);
                        break 'outer;
                    }
                }
            }
        }

        delone
            .triangles
            .retain(|t| bad_triangles.iter().all(|t_other| t != t_other));

        delone
    }
```

### Вывод
В ходе выполнения лабораторной работы была разработана графическая программа, выполняющая триангуляцию Делоне и построение диаграммы Вороного по заданному набору точек. Программа предоставляет удобный интерфейс с возможностью выбора метода построения через меню и панель инструментов.

Режим отладки позволяет наглядно отследить процесс построения триангуляции и диаграммы, что помогает в анализе работы алгоритмов. В результате:

* риангуляция Делоне обеспечила разбиение множества точек с хорошими свойствами.
* Диаграмма Вороного была успешно построена на основе триангуляции, корректно определяя области влияния каждой точки.

Таким образом, работа позволила изучить и реализовать важные алгоритмы вычислительной геометрии, а также их применение на практике.