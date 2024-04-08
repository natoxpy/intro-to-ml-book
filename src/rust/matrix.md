# Matrix Library

> [!NOTE]
> Making this is only encouraged for learning purposes, but if you want to build an efficient and fast machine learning model, I recommend you to use
> [ndarray](https://docs.rs/ndarray/latest/ndarray/) it has a lot more useful features.

In this page I will be implementing a Matrix in rust. For our purposes we only need a matrix with 2 axes.
And we only need to be able to perform 4 different operations.

\\[
\begin{split}
a &= \begin{bmatrix} 1 & 2 & 3 \\\\ 4 & 5 & 6 \end{bmatrix} \\
&c = \begin{bmatrix} 6 & 5 & 4 \\\\ 3 & 2 & 1 \end{bmatrix} \\\\
b &= \begin{bmatrix} 1 \\\\ 2 \\\\ 3 \end{bmatrix} \\
&d = \begin{bmatrix} 1 & 2 \\\\ 3 & 4 \\\\ 5 & 6 \end{bmatrix} \\\\
\end{split}
\\]

### Addition and Subtraction

This operation requires for both left and right hand side to have the same amount of items in each axis.
For example \\(a + b\\) or \\(b +d \\) would not be possible because they are not the same shape. But
\\( a+c \\) would be possible, this also applies for subtracting.

\\[
\begin{split}
a + c & =
\begin{bmatrix} 1 & 2 & 3 \\\\ 4 & 5 & 6 \end{bmatrix} +
\begin{bmatrix} 6 & 5 & 4 \\\\ 3 & 2 & 1 \end{bmatrix} =
\begin{bmatrix} 1 + 6 & 2 + 5 & 3 + 4 \\\\ 4 + 3 & 5 + 2 & 6 + 1 \end{bmatrix} \\\\
\\\\
a - c & =
\begin{bmatrix} 1 & 2 & 3 \\\\ 4 & 5 & 6 \end{bmatrix} -
\begin{bmatrix} 6 & 5 & 4 \\\\ 3 & 2 & 1 \end{bmatrix} =
\begin{bmatrix} 1 - 6 & 2 - 5 & 3 - 4 \\\\ 4 - 3 & 5 - 2 & 6 - 1 \end{bmatrix}
\end{split}
\\]

### Hadamard product

The hadamard product using the \\( \odot \\) symbol and it's similar to Addition and subtraction where both
left and right sides need to have an identical shape. For example \\( a \odot b \\) or \\(b \odot d \\) would
not be possible because they don't have the same shape. But \\(a \odot c\\) would be possible.

\\[
a \odot c =
\begin{bmatrix} 1 & 2 & 3 \\\\ 4 & 5 & 6 \end{bmatrix} +
\begin{bmatrix} 6 & 5 & 4 \\\\ 3 & 2 & 1 \end{bmatrix} =
\begin{bmatrix} 1 \cdot 6 & 2 \cdot 5 & 3 \cdot 4 \\\\ 4 \cdot 3 & 5 \cdot 2 & 6 \cdot 1 \end{bmatrix}
\\]

### Dot product

The dot product or just known as matrix multiplication has a different rule from the rest, the number of columns
on the left matrix has to be equal to the number of rows on the right matrix. For example \\( a\cdot c \\) and
\\(b \cdot c \\) would not be possible because the number of columns of the left side are not the same as the number of
rows on the right side. But \\(a \cdot d \\) would be possible.

\\[
\begin{split}
a \cdot d &= \begin{bmatrix} 1 & 2 & 3 \\\\ 4 & 5 & 6 \end{bmatrix} \cdot
\begin{bmatrix} 1 & 2 \\\\ 3 & 4 \\\\ 5 & 6 \end{bmatrix} \\\\
\\\\
&=
\begin{bmatrix}
\begin{bmatrix} 1 & 2 & 3 \end{bmatrix} \cdot \begin{bmatrix} 1 & 3 & 5 \end{bmatrix} &
\begin{bmatrix} 1 & 2 & 3 \end{bmatrix} \cdot \begin{bmatrix} 2 & 4 & 6 \end{bmatrix} \\\\
\begin{bmatrix} 4 & 5 & 6 \end{bmatrix} \cdot \begin{bmatrix} 1 & 3 & 5 \end{bmatrix} &
\begin{bmatrix} 4 & 5 & 6 \end{bmatrix} \cdot \begin{bmatrix} 2 & 4 & 6 \end{bmatrix}
\end{bmatrix} \\\\
\\\\
&=
\begin{bmatrix}
1 \cdot 1 + 2 \cdot 3 + 3 \cdot 5 & 1 \cdot 2 + 2 \cdot 4 + 3 \cdot 6 \\\\
4 \cdot 1 + 5 \cdot 3 + 6 \cdot 5 & 4 \cdot 2 + 5 \cdot 4 + 6 \cdot 6 \\\\
\end{bmatrix} \\\\
\\\\
&=
\begin{bmatrix}
22 & 28 \\\\
49 & 64
\end{bmatrix}
\end{split}
\\]

The formula for matrix multiplication:

\\[
\large{c_{ji} = \sum^n_{k=1}{a_{ik} b_{kj}}}
\\]

> [!tip]
> Checkout the wikipedia page on [matrix multiplication](https://en.wikipedia.org/wiki/Matrix_multiplication#:~:text=For%20matrix%20multiplication%2C%20the%20number,B%20is%20denoted%20as%20AB.)
> if you want to learn more.

That should be all the matrix math that we need to learn, now we can implement our own matrix in rust.

>

# Code

I'll be implementing an extra Vector struct to simplify some operations on the matrix struct.

```rust,noplayground
{{ #include matrix.rs:0:42 }}
```

## Addition and subtraction

```rust,noplayground
{{ #include matrix.rs:48:134 }}
```

## Hadamard product

```rust,noplayground
{{ #include matrix.rs:140:182 }}
```

## Matrix multiplication

```rust,noplayground
{{ #include matrix.rs:188:233 }}
```

## Display

```rust,noplayground
{{ #include matrix.rs:239:278 }}
```

## Macro

```rust,noplayground
{{ #include matrix.rs:284:310 }}
```

>

# Testing

`no output` means there were no errors.

```rust
{{ #rustdoc_include matrix.rs:421:425 }}
```
