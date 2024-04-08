# The Math for Machine Learning

> [!NOTE]
> I'm not a certified mathematician, so there's a possibility of incorrect math notation or misunderstandings in its usage.

Here, I'll present much of the mathematics used in ML in a way that will facilitate implementation as code later.

I'll be utilizing the Sigmoid activation function:

\\[
\begin{split}
\sigma(z) &= \frac{1}{1 + e^{-z}} \\\\
\sigma'(z) &= \sigma(z) (1 - \sigma(z))
\end{split}
\\]

With this cost function, where \\(x\\) is the output of the model and \\(y\\) is the expected value:

\\[
\begin{split}
C(x, y) &= \frac{1}{2} (y - x)^2 \\\\
C'(x, y) &= x - y
\end{split}
\\]

This represents basic forward propagation in a compressed form:

\\[
a^{l+1}\_k = \sigma(w_{kj} a^l_j + b_k)
\\]

And here's the backpropagation algorithm:

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

All the equations for backpropagation can be found in [Chapter 2](http://neuralnetworksanddeeplearning.com/chap2.html) of the book [Neural Networks and Deep Learning](http://neuralnetworksanddeeplearning.com/) by Michael Nielsen. Check it out for an in-depth explanation of all the mathematics condensed here.
