# Backpropagation

> [!NOTE]
> This page was generated with human assistance.

This page will explain the Backpropagation math shown before, which is key to how computers learn.

Backpropagation algorithm:

\\[
\begin{split}
\delta^L &= C'(a^{L}) \odot \sigma'(z^{L}) \\\\
\delta^l &= ((w^{l+1})^T \delta^{l+1}) \odot \sigma'(z^{l}) \\\\
\frac{\partial C}{\partial b^l_j} &= \delta^l_j \\\\
\frac{\partial C}{\partial w^{l}\_{jk}} &= \delta^l_j {(a^{l-1}\_k)}^T \\\\
w\_k \rightarrow w'\_k &= w_k - \frac{n}{m} \sum_n \frac{\partial C_{x_n}}{\partial w_{k}} \\\\
b_l \rightarrow b'\_l &= b_l - \frac{n}{m} \sum_n \frac{\partial C_{x_n}}{\partial b_l}
\end{split}
\\]

I will break down the algorithm into different parts and explain each separately, including notation meanings and examples.

### Understanding \\(\delta^L = C'(a^{L}) \odot \sigma'(z^{L}) \\)

\(\delta^L\) is calculated by taking the Hadamard product between \\(C'(a^L)\\) and \\(\sigma'(z^{L})\\), where \\(a^L\\) represents the activations for the last layer, and \\(z^{L}\\) represents the values from the last layer that have not been passed to the activation function. We can modify the equation learned for forward propagation into something that helps us here:

\\[
\begin{split}
z^L_k &= w^L_{kj} a^{L - 1}_j + b^L_k \\\\
a^L_k &= \sigma(z^L)
\end{split}
\\]

### How \\(\delta^L\\) and \\(\delta^l\\) are related

In Backpropagation, we start from the output layer and move backwards until reaching the input layer. \\(\delta^L\\) is our starting point, where the index \\(L^{th}\\) indicates the last layer in the neural network. After calculating \\(\delta^L\\) with the given equation, we need to use the equation for \\(\delta^l\\) to calculate the \\(\delta^{L - 1}\\) layer:

\\[
\begin{split}
\delta^{L} &= C'(a^{L}) \odot \sigma'(z^{L}) \\\\
\delta^{l} &= ((w^{l + 1})^T \delta^{l + 1}) \odot \sigma'(z^{l})
\end{split}
\\]

### Understanding \\(\delta^{l} = ((w^{l + 1})^T \delta^{l + 1}) \odot \sigma'(z^{l})\\)

For this equation, to calculate \\(\delta^{l}\\), we need \\(w^{l+1}\\) (weights for the next layer), \\(\delta^{l+1}\\) (from the next layer), and \\(z^{l}\\) (of the same layer). Since we need variables from the future, we cannot use this equation to calculate the last layer, which is why we have a separate equation for the output layer. From there, we can move backwards.

### What are \\(l\\) and \\(L\\)?

In normal math, this would indicate to the power of \\(l\\), but here, it represents the index for the layer. \\(l\\) refers to the current layer, while \\(L\\) indicates the last layer. \\(L\\) starts counting from the end, with \\(l\\) representing the first layer. For instance, \\(L^{th}\\) denotes the last layer, \(l^{th}\) represents the current layer, and \\((L - 1)^{th}\\) indicates the next-to-last layer.

### Applying calculated changes

After calculating the gradients for weights and biases using backpropagation, we apply these changes to update the parameters of the neural network.

### Updating the Weights

For each weight \\(w_k\\), we update it using the formula:

\\[
w'\_k = w_k - \frac{n}{m} \sum_{n} \frac{\partial C_{x_n}}{\partial w_{k}}
\\]

Here, \(n\) iterates over all the training examples, \(m\) is the total number of training examples, \(C\_{x_n}\) is the cost function for the \(n^{th}\) training example, and \(\frac{\partial C\_{x_n}}{\partial w\_{k}}\) is the partial derivative of the cost function with respect to the weight \(w_k\) for the \(n^{th}\) example.

### Updating the Biases

Similarly, for each bias \(b_l\), we update it using the formula:

\\[
b'\_l = b_l - \frac{n}{m} \sum_{n} \frac{\partial C_{x_n}}{\partial b_l}
\\]

Here, \\(n\\), \\(m\\), and \\(C\_{x_n}\\) have the same meanings as described above, and \\(\frac{\partial C\_{x_n}}{\partial b_l}\\) is the partial derivative of the cost function with respect to the bias \\(b_l\\) for the \\(n^{th}\\) example.

These formulas essentially update the parameters (weights and biases) of the neural network in the opposite direction of the gradient of the cost function with respect to each parameter. The factor \\(\frac{n}{m}\\) acts as a scaling factor, where \\(n\\) is the learning rate and \\(m\\) is the total number of training examples. This scaling ensures that the updates are appropriate regardless of the size of the training set.

In summary, the backpropagation algorithm allows the neural network to adjust its parameters iteratively, leading to improved performance in subsequent training epochs until convergence.
