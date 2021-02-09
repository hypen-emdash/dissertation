# Formalising the Problem

## Input

The input is a complete, weighted, undirected graph $G = (V, E)$, and a natural number $n$.

+ $V$ is the guest list.
+ Between any two guests $u$ and $v$, there is an integer weighting $E_{u, v}$. $0$ if they have never met, positive if they have a positive relationship and negative if they have a negative relationship. For example, If two people simply know each other, they have a weight of $1$; if they're romantically involved they have a weight of $50$. Each guest has a self-relationship of $0$.
+ $n$ is the number of tables. It must divide the number of guests.

## Output

The output $P$ is a partition of $V$ with $n$ bins. Each bin must be of cardinality $\frac{V}{n}$.

## Comparison

The first objective is that everyone should be sat with at least one person they have a positive relationship with. If a seating chart fails this it earns a score of $lonely(g)$ where $g$ is the number of guests without a friend nearby. If this objective is met, the score is $befriended(h)$ where $h$ is a measure of the total happiness, calculated as

$h = \sum_{p \in P} \sum_{u, v \in p} E_{u, v}$

(For each person, calculate their individual happiness as the sum of their relationships with the people at their table. Total happiness is the sum of individual happiness.)

Scores are compared with the following (reasonably intuitive) scheme:

+ All befriended scores beat all lonely scores.
+ If both scores are lonely, the one with the lower number of loney guests is considered better.
+ If both scores are befriended, the one with the higher happiness is considered better.
