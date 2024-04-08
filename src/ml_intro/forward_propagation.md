# Forward propagation

In the previous chapter we looked at the forward propagation function.
In this page I'll be explaining how it works.

This is the forward propagation formula.
\\[
a^{l+1}\_k = \sigma(w^l\_{kj} a^l_j + b^l_k)
\\]

To visualize each variable in that equation: \\( w^{l}\_{kj} \\) represents all the weights for any given \\(l \\)th layer,
\\( a^l\_{j} \\) represents all activation values for any given \\( l \\)th layer, and \\( b^l_k \\) represents the bias
values for any given \\( l \\)th layer.

In the following example, we have a neural network with 2 layers: an input layer (the green circles) and the output layer (the grey circles).
Each circle is called a neuron. The input layer has four neurons, while the output has only two.
The variable \\( a^l_j \\) corresponds to the input layer, and the variable \\( w^l\_{kj} \\) corresponds to all the weights between the input and output layers.
Finally, \\( b^l_k \\) applies to each of the neurons in the output layer.

\\[
\begin{split}
w^l\_{kj} &= \begin{bmatrix} 1 & 2 & 3 & 4 \\\\ 5 & 6 & 7 & 8 \end{bmatrix}
a^l_j = \begin{bmatrix} 1 \\\\ 2 \\\\ 3 \\\\ 4 \end{bmatrix}
b^l_k = \begin{bmatrix} 1 \\\\ 2 \end{bmatrix} \\\\
\end{split}
\\]

<p align="center">
    <img src="/images/visual_representation_of_variables.png">
</p>

> [!NOTE]
> It's worth noting that 2 layers is the minimum amount of layers for a neural network,
> I'm only showing 2 layers for simplicity, but most neural networks have more than 2 layers.

>

# How to use the forward propagation formula

Now that we have values for variables \\( w^l\_{kj} \\), \\( a^l\_{j} \\), and \\( b^l\_{k} \\), we can calculate \\( a^{l+1}\_{j} \\),
which represents the next layer in the neural network.

To calculate the value of \\( a^{l + 1}\_0\\) for the output layer we can do all the math as follows.

\\[
\begin{split}
a^{l+1}\_0 &= \sigma \left( \left(1 \cdot 1 + 2 \cdot 2 + 3 \cdot 3 + 4 \cdot 4 \right) + 1 \right) = \sigma \left(30 + 1 \right)\\\\
a^{l+1}\_1 &= \sigma \left( \left(5 \cdot 1 + 6 \cdot 2 + 7 \cdot 3 + 8 \cdot 4 \right) + 2 \right) = \sigma \left( 70 + 2 \right) \\\\
\end{split}
\\]

We can replace all of that with our variables from before, and proper indexing.
\\[
\begin{split}
a^{l+1}\_0 &= \left(w^l\_{00} a^l\_0 + w^l\_{01} a^l\_1 + w^l\_{02} a^l\_2 + w^l\_{03} a^l\_3 \right) + b^l\_0 \\\\
a^{l+1}\_1 &= \left(w^l\_{10} a^l\_0 + w^l\_{11} a^l\_1 + w^l\_{12} a^l\_2 + w^l\_{13} a^l\_3 \right) + b^l\_1
\end{split}
\\]

All of that can be simplified using matrix multiplication between \\( w^l\_{kj} \\) and \\( a^l\_{j} \\) and finally adding \\( b^l\_{k} \\).

\\[
\begin{split}
a^{l+1}\_k &=
\sigma \left(
\begin{bmatrix} 1 & 2 & 3 & 4 \\\\ 5 & 6 & 7 & 8 \end{bmatrix}
\cdot \begin{bmatrix} 1 \\\\ 2 \\\\ 3 \\\\ 4 \end{bmatrix} + \begin{bmatrix} 1 \\\\ 2 \end{bmatrix} \right) \\\\
\\\\
a^{l+1}\_k &= \sigma \left( \begin{bmatrix} 31 \\\\ 72 \end{bmatrix} \right) = \begin{bmatrix} \sigma (31) \\\\ \sigma (72) \end{bmatrix}
\\\\
\end{split}
\\]

<p align="center">
    <img src="/images/NN_output_calculated.png">
</p>

With this we know that given the input \\( a^l = \begin{bmatrix} 1 & 2 & 3 & 4 \end{bmatrix} \\) this basic neural network will give us the result
\\( a^{l+1} = \begin{bmatrix} \sigma (31) & \sigma (72) \end{bmatrix} \\) for the output layer. And you can repeat this for any given number of layers.
To compute \\(a^{l+2} \\) the process is the same, the equation would be \\( a^{l+2} = \sigma(w^{l+1} a^{l+1} + b^{l+1}) \\).

> [!NOTE]
> I don't compute \\( \sigma(z) \\) for those values because when the inputs are 31 and 72, the sigmoid function returns 1. Having both outputs as 1 could be confusing.
