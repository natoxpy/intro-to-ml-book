# Backpropagation

This page will be explaining all of the Backpropagation math shown before. This is the key to how computers learn.

Backpropagation algorithm:
\\[
\begin{split}
\delta^L &= C'(a^{L}) \odot \sigma'(z^{L}) \\\\
\delta^l &= ((w^{l+1})^T \delta^{l+1}) \odot \sigma'(z^{l}) \\\\
\frac{\partial C}{\partial b^l_j} &= \delta^l_j \\\\
\frac{\partial C}{\partial w^{l}\_{jk}} &= \delta^l_j {(a^{l-1}\_k)}^T \\\\
w\_k \rightarrow w\'\_k &= w_k - \frac{n}{m} \sum_n \frac{\partial C_{x_n}}{\partial w_{k}} \\\\
b_l \rightarrow b'\_l &= b_l - \frac{n}{m} \sum_n \frac{\partial C_{x_n}}{\partial b_l}
\end{split}
\\]

I will be breaking down the algorithm into different parts and explaining all of them separately.
The explanation will go into what the notation means, and what the indexing means, and include examples.

>

### Understanding \\(\delta^L = C'(a^{L}) \odot \sigma'(z^{L}) \\)

\\(\delta^L \\) is calculated by taking the Hadamard product between \\(C'(a^L) \\) and \\(\sigma'(z^{L}) \\) where
\\(a^L \\) are the activations for the last layer, and \\(z^{L} \\) the values from the last layer that has not being pass to the activation function.
We can modify the equation we learned for forward propagation into something that will helps us here.

\\[
\begin{split}
z^L\_k &= w^L\_{kj} a^{L - 1}_j + b^L_k \\\\\
a^L\_k &= \sigma(z^L)
\end{split}
\\]

### How are \\(\delta^L \\) and \\(\delta^l \\) related

As the name implies in Backpropagation, we start from the output layer, and move backwards until we reach the input layer.
\\(\delta^L \\) is our starting point, where the index \\(L^{th}\\) tells us this is the last layer in the neural network.
After we calculated \\(\delta^L \\) with the equation we have, we need to use the equation given for \\(\delta^l \\) to calculate
the \\(\delta^{L - 1} \\) layer.

\\[
\begin{split}
\delta^{L} &= C'(a^{L}) \odot \sigma'(z^{L}) \\\\
\delta^{l} &= ((w^{l + 1})^T \delta^{l + 1}) \odot \sigma'(z^{l})
\end{split}
\\]

### Understanding \\(\delta^{l} = ((w^{l + 1})^T \delta^{l + 1}) \odot \sigma'(z^{l})\\)

The indexing for this one can be weird, but basically it is telling us how, to calculate \\(\delta^{l} \\) we need to have
\\(w^{l+1}\\) which are the weights for the next layer, the \\(\delta^{l+1}\\) from the next layer, and the \\(z^{l} \\) of the same layer.
And because you need 2 variables from the future, we cannot use it to calculate the last layer. That is why we have a separate equation for the output layer.
And from there we can move backwards.

### What is the \\( l\\) and \\(L \\)?

In normal math this would be, to the power of \\( l\\) but here this is the index for the layer.
Meaning that if you see anything raised to the power of \\(l \\) it means the \\(l^{th} \\) layer on the neural network.
The index \\(L \\) is the same as \\(l \\) but starting from the end. In the \\(l^{th} \\) layer we assume this to be the first layer,
the second being the \\((l+1)^{th} \\) layer. Now we can say that \\(L^{th} \\) is the last layer. To get the next to last layer
you would write it as the \\((L - 1)^{th} \\) layer.

>

# Applying calculated changes

Backpropagation algorithm final part:
\\[
\begin{split}
w\_k \rightarrow w\'\_k &= w_k - \frac{n}{m} \sum_n \frac{\partial C_{x_n}}{\partial w_{k}} \\\\
b_l \rightarrow b'\_l &= b_l - \frac{n}{m} \sum_n \frac{\partial C_{x_n}}{\partial b_l}
\end{split}
\\]
