# ECC Operations

本项目为去年斌头安全学实验之一, 本项目不考虑运行效率, 全旨在将各个ECC操作变得通用.

## Field

为了实现通用的`ECC`操作, 首先需要抽象出域*Field*的概念, 得益于`Rust`的`trait bound`, 这些都很容易被抽象出来. 以下是`Field`接口的定义:

```rust
pub trait Field:
    Sized
    + Clone
    + Zero // 加法单位元
    + One  // 乘法单位元
    + Add<Output = Self> // 加法封闭
    + Sub<Output = Self> // 加法逆元
    + Mul<Output = Self> // 乘法逆元
    + Div<Output = Self> // 乘法封闭
    + PartialEq
{
}

impl<T> Field for T where
    T: Sized
        + Clone
        + Zero
        + One
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + PartialEq
{}
```

接下来, 我们很容易就可以实现各种域: 例如`GF(2^8), Zp*`等. 只需要分别实现`Field`要求的`trait bound`即可. 

## Poly

首先我们需要实现一个`GF(2^8)`, 通过位运算我们可以很快实现一个运行效率很快的版本, 但是为了可以使我们的域拓展到`GF(2^n)`, 我们需要抽象出一个通用的多项式结构.

```rust
#[derive(Debug, Clone)]
pub struct Poly<T> {
    data: VecDeque<T>, // 存放系数
}
```

多项式的加减乘依赖于系数的加减乘. 这个时候Rust的`trait bound`又开始发挥作用了. 举个例子, 我们可以实现这样的一个多项式加法:

```rust
impl<T> Add for Poly<T>
where
    T: Zero + Add<Output = T> + Clone,
{
    type Output = Self;
    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.align(&mut rhs, T::zero()); // 多项式长度对其
        Poly { // 系数相加后返回结果
            data: self
                .data
                .into_iter()
                .zip(rhs.data)
                .map(|(x, y)| x + y)
                .collect(),
        }
    }
}
```

但是, 多项式的除法需要更加复杂的`trait bound`. 我们的系数必须是满足`Field`bound的. 如果系数在某个域中, 我们就能够定义以下的除法. 当然, 我们需要先定义出多项式的位移运算, 这个很容易可以实现, 不再赘述, 下边是小学生除法的代码实现...

```rust
impl<T> DivRem for Poly<T>
where
    T: Field,
{
    type Output = Self;
    fn divrem(self, rhs: Self) -> (Self::Output, Self::Output) {
        if self == Self::zero() {
            (Self::zero(), Self::zero())
        } else {
            let (q, r) = (self.clone() >> 1).divrem(rhs.clone());
            let (q, r) = (q << 1, r << 1);
            let r = r + self.sub_item(0);

            if r.order() >= rhs.order() {
                let c =
                    T::one() / rhs.last_coefficient() * r.last_coefficient();
                let c = Self::new(vec![c]);
                return (q + c.clone(), r - rhs * c);
            }
            (q, r)
        }
    }
}
```

至此, `Poly`基本上做完了. 在上边的基础上可以写出多项式的`egcd`:

```rust
impl<T> Egcd for Poly<T>
where
    T: Field,
{
    type Output = Self;
    fn egcd(self, rhs: Self) -> (Self::Output, Self::Output) {
        if rhs == Self::zero() {
            (Self::one(), Self::zero())
        } else {
            let (q, r) = self.divrem(rhs.clone());
            let (x, y) = rhs.egcd(r);
            (y.clone(), x - q * y)
        }
    }
}
```



## GF(2^8)

`GF(2^8)`的实现已经没有难度了, 上边的运算都做好了, 只要构造出相应的系数就好了, 详见`bit_wrapper.rs`

## Zp*

好像也没有难度了?...

## Ec

接下来就要抽象出一个`Ec trait`, 回顾一下, 我们需要在`Ec`做以下操作:

1. 点加
2. 取反
3. 乘法

所以可以有:

```rust
pub trait Ec<F: Field> {
    fn add(&self, p: Point<F>, q: Point<F>) -> Option<Point<F>>;
    fn neg(&self, p: Point<F>) -> Option<Point<F>>;
    fn mul(&self, n: isize, p: Point<F>) -> Option<Point<F>>;
}
```

感谢`Rust`的代数枚举, 治好了我多年的强迫症, 现在我们可以愉快的定义`Ec`上的点了

```rust
#[derive(PartialEq, Clone, Eq, Debug)]
pub enum Point<F: Field> {
    Identity, // 无穷原点
    Ordinary(F, F), // 普通的点
}
```

```rust
// 顺手来个constructor
impl<F: Field> Point<F> {
    pub fn new(x: F, y: F) -> Self {
        Point::Ordinary(x, y)
    }
}
```



## E256

接下来只要实现一下点加, 点取反和点乘法就好了, 都是套公式的细节, 不再赘述.

然后我们来看看我们的一些成果:

```rust
// 打印出E(23,107)上边所有的点
fn main() {
    let a = Gf256::from(23);
    let b = Gf256::from(107);
    let ec = E256::new(a, b);
    ec.points().par_iter().for_each(|p| {
        println!(
            "{}",
            match p {
                Point::Identity => "".to_string(),
                Point::Ordinary(x, y) => {
                    format!("{},{}", u8::from(x.clone()), u8::from(y.clone()))
                }
            }
        );
    })
}
```



