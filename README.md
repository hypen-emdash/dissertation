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

Solutions are optimised to achieve maximum total happiness, calculated by the formula $\sum_{p \in P} \sum_{u, v \in p} E_{u, v}$.

Other measures that are "nice to have" but not known to the solvers include minimum and median happiness of an individual, and the number of people with no positive relations at their table.
