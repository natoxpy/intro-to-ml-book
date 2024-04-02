# Forward propagation

In the previous chapter we looked at the forward propagation function.
In this page I'll be explaining how it works.

This is the forward propagation formula.
\\[
a^{l+1}\_k = \sigma(W\_{kj} a^l_j + b_k)
\\]

To visualize what each variable in that equation looks like, I'll assign 1 to all of them
\\[
\begin{split}
w^l_{kj} &= \begin{bmatrix} 1 & 1 & 1 \\\\ 1 & 1 & 1 \end{bmatrix}
a^l_j = \begin{bmatrix} 1 \\\\ 1 \end{bmatrix}
b^l_k = \begin{bmatrix} 1 \\\\ 1 \end{bmatrix} \\\\
\\\\
a^{l+1}_k &=
\begin{bmatrix} 1 & 1 & 1 \\\\ 1 & 1 & 1 \end{bmatrix}
\cdot \begin{bmatrix} 1 \\\\ 1 \\\\ 1 \end{bmatrix} + \begin{bmatrix} 1 \\\\ 1 \end{bmatrix} =
\begin{bmatrix} 4 \\\\ 4 \end{bmatrix}
\\\\
\end{split}
\\]

<p align="center">
    <img src="/images/test.png">
</p>
