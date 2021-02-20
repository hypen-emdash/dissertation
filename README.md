# Wedding Seating Plan

## Formalising the Problem

### Input

The input is a complete, weighted, undirected graph $G = (V, E)$, and a natural number $n$.

+ $V$ is the guest list.
+ Between any two guests $u$ and $v$, there is an integer weighting $E_{u, v}$. $0$ if they have never met, positive if they have a positive relationship and negative if they have a negative relationship. For example, If two people simply know each other, they have a weight of $1$; if they're romantically involved they have a weight of $50$. Each guest has a self-relationship of $0$.
+ $n$ is the number of tables. It must divide the number of guests.

### Output

The output $P$ is a partition of $V$ with $n$ bins. Each bin must be of cardinality $\frac{V}{n}$.

### Comparison

There are two objectives:

+ Minimise the number of people who are sat without any positive relationships nearby.
+ Maximise the total happiness according to the formula: $\sum_{p \in P} \sum_{u, v \in p} E_{u, v}$

The first objective is considered strictly more important. If at all possible, the number of lonely guests should be zero.

The second objective models total happiness as the sum total of all the guests' individual happiness, which is modelled as the sum total of their relationships with the people at their table.
