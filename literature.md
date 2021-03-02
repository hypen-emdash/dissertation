# Literature Review

## Optimisation Background

_Christodoulos A. Floudas_

https://ebookcentral.proquest.com/lib/abdn/detail.action?docID=430844

+ Geometric definitions

+ Optimisation definitions
  
  + Begins with continuous variables
  
  + local/global extrema
  
  + min/max/saddle stationary points
  
  + There are necessary and sufficient conditions for optimality concerning curvature and convexity.

## Integer Programming

https://developers.google.com/optimization/mip/mip

Not enough to solve for the continuous case and then round to the nearest integer.

Phrase the problem as optimising a linear function with linear constraints, then use one of several backends to mathemagically find a solution.

## Genetic Algorithm

https://ebookcentral.proquest.com/lib/abdn/reader.action?docID=5015534&ppg=79

Keep a population of potential solutions and evaluate them according to how well they solve the problem (fitness). Good solutions reproduce at a higher rate than bad. Reproduction normally involves crossover between two "parents", and occasionally random mutations. Overall fitness improves over time.

Terminate when you've spent enough time, done enough generations, or found a good-enough solutions.

There are many schemes for deciding which solutions get to reproduce, and with what partner; most involve randomness. Broadly categorised into _absolute_ and _comparative_. Absolute schemes look at the precise fitness value (assumed to be a real number), while comparative schemes only compare fitnesses to each other. Absolute acts differently with $f$ and $e^f$ as fitness functions, while comparative doesn't.

| Absolute     | Comparitive |
| ------------ | ----------- |
| Proportional | Ranking     |
|              | Tournament  |

## Hill Climbing

https://www.edureka.co/blog/hill-climbing-algorithm-ai/

Basic idea is to keep track of a single solution and loop it through an algorithm that makes changes to it so that it generally improves over time. There are a few methods:

+ Make a random change and compare it to the current solution. Accept the change iff it improves things.

+ Generate all possible random changes and take the best one.

+ Simulated annealing. Pick a random change. Take it if it improves things, and take it with some small probability if it doesn't.

The core algorithm is very bad at improving from local optima. To overcome this, you first have to detect when you're in one, and then either make a large jump, or backtrack and take a different path.

### Late Acceptance

https://www.sciencedirect.com/science/article/pii/S0377221716305495

Like normal hill-climbing, but accepts a move if the result is better than our solution from _n_ iterations ago. (n is fixed.)

A good hill-climbing derivative is _simulated annealing_, which accepts worse changes with small probability: `exp((curent - candidate) / temperature)` where termperature generally decreases over time (it can increase, but such algorithms are weird).

Other variations include _threshold accepting_ and the _great deluge algorithm_.

+ Paper claims it to be better than greedy hill-climbing, simulated annealing, threshold acceptance, and great deluge.

+ Depends on a single algorithmic parameter (I'm guessing how much history to look at.)

+ > This suggests its stronger reliability on [non-linear problems](https://www.sciencedirect.com/topics/mathematics/nonlinear-problem "Learn more about Nonlinear Problem from ScienceDirect's AI-generated Topic Pages") and this could be beneficial in new application areas.

A fixed-size queue of candidates/fitnesses (I guess either could work) is kept, and new candidate solutions are compared against the newest and oldest in the queue.

Formula 1: `accept iff candidate better than oldest OR candidate better than newest`

Initialisation of the queue does not seem to make much difference.

Simulates greedy hill-climbing towards end of algorithm.

For the sake of this paper, algorithm terminates when it stagnates.

They work with a history length of 5000 to 50000. CPU time seems to be proportional to history length.

Won 1st place at international optimisation competition 2011 on magic squares.

## Hungarian Algorithm

Solves the assignment problem:

+ There are agents and tasks. Each agent's suitability for each task is quantified.

+ Assign tasks to agents in a 1:1 fashion.

+ Minimise total cost.

Not quite relevant, as 1:1 relation of assignment.
