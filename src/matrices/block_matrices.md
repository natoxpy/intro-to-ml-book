# Block matrices

The concept of a block matrix, or partitioned matrix is most useful for matrix multiplication, so I will only be diving into that here.

We will start with the following matrices.

\\[
\begin{split}
A = \left\[
\begin{array}{cc}
1 & 2 & 3 & 4 & 5 & 6 \\\\
2 & 2 & 3 & 4 & 5 & 6 \\\\
3 & 2 & 3 & 4 & 5 & 6 \\\\
4 & 2 & 3 & 4 & 5 & 6 \\\\
5 & 2 & 3 & 4 & 5 & 6 \\\\
6 & 2 & 3 & 4 & 5 & 6
\end{array} \right\]
\\\\ \\\\
B = \left\[
\begin{array}{cc}
1 & 2 & 3 & 4 & 5 & 6 \\\\
2 & 2 & 3 & 4 & 5 & 6 \\\\
3 & 2 & 3 & 4 & 5 & 6 \\\\
4 & 2 & 3 & 4 & 5 & 6 \\\\
5 & 2 & 3 & 4 & 5 & 6 \\\\
6 & 2 & 3 & 4 & 5 & 6
\end{array} \right\]
\end{split}
\\]

Now let's perform matrix multiplication on those two matrices, what is \\( A \cdot B \stackrel{?}{=} C\\)
or wait, that would take about 216 multiplications and 180 additions, or 396 total calculations. If we had 2 100x100 matrices it would take
1,000,000 multiplications and 990,000 additions. And if we had two 10,000x10,000 matrices it would take 1,000,000,000,000 multiplications and,
999,900,000 additions.

Given matrix of size \\( n \times m \\) and \\( m \times p \\) we can calculate the total multiplications with \\( n \cdot p \cdot m \\) and the total additions with
\\( n \cdot p \cdot (m - 1) \\) with this we get that, given square matrices, total number of calculations required to multiply two square matrices is
\\( 2n^3 - n^2 \\) which is equivalent to \\(n \cdot p \cdot (2m - 1) \\)

Now there is our issue, our cost to compute grows exponentially. Computing Everything in one sitting would be a nightmare. Even for a computer given a large enough matrix.
Based on my testing, computing matrix multiplication between 300x300 matrices can be quite expensive to compute, after all the computer is trying to perform 53,910,000 calculations.

>

# But what is block matrix multiplication?

Imagine the matrices from before, but this time, draw two lines in between them.

\\[
\begin{split}
A = \left\[
\begin{array}{cc|cc|cc}
1 & 2 & 3 & 4 & 5 & 6 \\\\
2 & 2 & 3 & 4 & 5 & 6 \\\\ \hline
3 & 2 & 3 & 4 & 5 & 6 \\\\
4 & 2 & 3 & 4 & 5 & 6 \\\\ \hline
5 & 2 & 3 & 4 & 5 & 6 \\\\
6 & 2 & 3 & 4 & 5 & 6
\end{array} \right\]
\\\\ \\\\
B = \left\[
\begin{array}{cc|cc|cc}
1 & 2 & 3 & 4 & 5 & 6 \\\\
2 & 2 & 3 & 4 & 5 & 6 \\\\ \hline
3 & 2 & 3 & 4 & 5 & 6 \\\\
4 & 2 & 3 & 4 & 5 & 6 \\\\ \hline
5 & 2 & 3 & 4 & 5 & 6 \\\\
6 & 2 & 3 & 4 & 5 & 6
\end{array} \right\]
\end{split}
\\]

And now replace those sections with \\( A\_{ij} \\) and \\( B\_{ij} \\) you should end up with something like this

\\[
\begin{split}
A = \left\[
\begin{array}{cc}
A_{11} & A_{12} & A_{13} \\\\
A_{21} & A_{22} & A_{23} \\\\
A_{31} & A_{32} & A_{33}
\end{array} \right\]
\\\\ \\\\
B = \left\[
\begin{array}{cc}
B_{11} & B_{12} & B_{13} \\\\
B_{21} & B_{22} & B_{23} \\\\
B_{31} & B_{32} & B_{33}
\end{array} \right\]
\end{split}
\\]

Now we went from \\(6 \times 6 \\) matrices to having nine \\(2 \times 2 \\) matrices inside each of our matrices.

\\[
\left\[
\begin{array}{cc|cc}
1 & 2 & 3 & 4 \\\\
1 & 2 & 3 & 4 \\\\ \hline
5 & 6 & 7 & 8 \\\\
5 & 6 & 7 & 8
\end{array} \right\]
\\]
