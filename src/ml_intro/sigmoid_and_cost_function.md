# The cost function

In the math introduction page we saw my cost and activation function of choice. Here I'll be explaining how to use it and how it will be relevant later
for Backpropagation.

Cost function:
\\[
\begin{split}
C(z) &= \frac{1}{2} (y - x)^2 \\\\
C'(z) &= x - y
\end{split}
\\]

The cost function takes in 2 parameter, \\(x \\) is the output of our model and \\(y \\) is the value we expected.
The higher the value we get from \\(C(z) \\) the worse our AI model performed.

Machine learning is all about computing \\( \frac{\partial C}{\partial w} \\) and \\( \frac{\partial C}{\partial b} \\). In simpler words, computing the rate at which
the cost function changes with respect the weights and the biases. With that we can know how to modify those values for our neural network in a way that
it will decrease the cost function to its local lowest value. And that is to say, if we decrease the cost function it will approximate more the value we expect and want
from our neural network.

Keep in mind that to calculate those values for weights and biases we have to take an average accross our entire training data. A little trick we use to optimize this process
is to turn all of our data into smaller chunks which are faster to compute.

This will give us our average cost. Later we will learn how to calculate \\( \frac{\partial C}{\partial w} \\) and \\( \frac{\partial C}{\partial b} \\).
\\[
\begin{split}
C(z) &= \frac{1}{n} \sum^n \frac{1}{2} (y - x)^2 \\\\
\end{split}
\\]

This function will normalize any output we give it and turn it into some number between 0 or 1, 0 being a very negative number, 1 being a very positive number.

Sigmoid function (Activation function):
\\[
\begin{split}
\sigma(z) &= \frac{1}{1 + e^{-z}} \\\\
\sigma'(z) &= \sigma(z) (1 - \sigma(z))
\end{split}
\\]

> [!NOTE]
> I'm not entirely sure exactly how important is having an activation function, as of now I'm still doing research on the topic.
