## Algorithms of Deception

Reporters observe numbers sampled from a distribution and make reports, the Audience tries to infer the distribution from the reports. If the Reporters just report what they see, that's most efficient. If Reporters have an incentive to make the Audience infer a distribution other than the real one, what happens depends on the specific algorithms the Reporters and the Audience are using.

 * *Lying*: if the reporters have no compunction about lying, that's game over. (If the listeners don't _know_ you're lying, then the reporter can make them believe anything she wants. If the listeners _do_ know, then they can disregard reports as noise.)
 
 * [*Selective reporting*](https://www.lesswrong.com/posts/DoPo4PDjgSySquHX8/heads-i-win-tails-never-heard-of-her-or-selective-reporting): if the Audience doesn't know the Reporter's algorithm, then the Reporter can make them believe _almost_ anything they want (as long as they have enough observations and the distribution doesn't vanish in the wrong places). If the Audience _does_ know the reporting algorithm, they won't be misled, but communication is less efficient.

 * [*Conceptual gerrymandering*](https://www.lesswrong.com/posts/esRZaPXSHgWzyB2NL/where-to-draw-the-boundaries): if the Reporter doesn't share observations _directly_, but rather "binned" summaries, then a bad choice of bins can be misleading.

To make this simple, let's assume Reality is a binomial distribution. (Because I want something discrete to make the computations easier on me, and it's normal in the limit.) The Audience can start out with a distribution on the unknown parameter _p_, and then update it.

... except that in order to be able to do "conceptual gerrymandering", I think I need non-binary events. It should be a categorical/multinomial distribution.

This project could be an exciting way for me to actually learn about conjugate priors, the Dirichlet distribution, _&c._!
