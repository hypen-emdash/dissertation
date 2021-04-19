#!py/venv/bin/python

import json
import sys

from dataclasses import dataclass
from typing import List
from ortools.linear_solver import pywraplp


@dataclass
class Problem:
    guest_relations: List[int]
    n_tables: int


class Solution:
    def __init__(self, solver, status, variables):
        self.solver = solver
        self.status = status
        self.variables = variables


def main():
    problem = get_problem()
    solution = plan_dinner(problem.guest_relations, problem.n_tables)
    display_sol(solution)


def plan_dinner(guest_relations, n_tables):
    """Create a seating plan to make people happy.

    args:
    * guest_relations: a list of lists signifying relationship between guests i and j.
                       should be square-dimensioned, integer-valued, symmetric, and 0
                       along the diagonal.
    * n_tables: the number of tables. Should evenly divide the number of guests.
    """

    assert (
        len(guest_relations) % n_tables == 0
    ), "Partial filling of tables not yet supported."

    solver = pywraplp.Solver.CreateSolver("SCIP")

    n_guests = len(guest_relations)
    table_size = int(n_guests / n_tables)

    variables = Variables(solver, n_guests, n_tables)
    add_constraints(solver, variables)
    solver.Maximize(get_objective(guest_relations, variables))

    status = solver.Solve()

    return Solution(solver, status, variables)


class Variables:
    def __init__(self, solver, n_guests, n_tables):
        # indexed [table][guest]
        self.at_table = [
            [solver.IntVar(0, 1, f"at_table[{i}{j}]") for j in range(n_guests)]
            for i in range(n_tables)
        ]

        # indexed [table][guest1][guest2]
        self.pair_at_table = [
            [
                [
                    solver.IntVar(0, 1, f"pair_at_table[{i}][{j}][{k}]")
                    for k in range(n_guests)
                ]
                for j in range(n_guests)
            ]
            for i in range(n_tables)
        ]

        self.n_guests = n_guests
        self.n_tables = n_tables


def get_objective(guest_relations, variables):
    obj = 0
    for i in range(variables.n_tables):
        for j in range(variables.n_guests - 1):
            for k in range(j + 1, variables.n_guests):
                obj += guest_relations[j][k] * variables.pair_at_table[i][j][k]
    return obj


def add_constraints(solver, variables):
    max_at_table = int(variables.n_guests / variables.n_tables)

    # original paper | eq 2 - a guest must be seated at exactly one table.
    for j in range(variables.n_guests):
        tables_seated_at = 0
        for i in range(variables.n_tables):
            tables_seated_at += variables.at_table[i][j]
        solver.Add(tables_seated_at == 1)

    # original paper | eq 3 - a table can only fit so many people.
    for i in range(variables.n_tables):
        people_seated = 0
        for j in range(variables.n_guests):
            people_seated += variables.at_table[i][j]
        solver.Add(people_seated <= max_at_table)

    # original paper | eq 5 - join the two types of variables. Scale of eq 3.
    for i in range(variables.n_tables):
        for k in range(variables.n_guests):
            lhs = 0
            for j in range(variables.n_guests):
                lhs += variables.pair_at_table[i][j][k]
            solver.Add(lhs <= max_at_table * variables.at_table[i][k])

    # original paper | eq 6 - mirror of eq 5.
    for i in range(variables.n_tables):
        for j in range(variables.n_guests):
            lhs = 0
            for k in range(variables.n_guests):
                lhs += variables.pair_at_table[i][j][k]
            solver.Add(lhs <= max_at_table * variables.at_table[i][j])


def get_problem():
    problem_json = json.load(sys.stdin)
    problem = Problem(problem_json["relations"]["relationships"], problem_json["n_tables"])

    # We require positive weights only for linearisation, otherwise the algorithm can
    # just lie and ignore the fact that two people are sat next to each other.

    worst = min(min(rs) for rs in problem.guest_relations)
    if worst < 0:
        shift = abs(worst)
        for rs in problem.guest_relations:
            for i in range(len(rs)):
                rs[i] += shift
    
    return problem


def display_sol(solution):

    # The current solution is a list of tables, where each table has a 1 or 0 depending
    # on whether that guest is there or not.
    # We convert that to a list of tables, where each table is a list of integers,
    # representing guest-ids.

    tables_present = [
        [j for j, present in enumerate(table_binary) if present.solution_value() > 0]
        for i, table_binary in enumerate(solution.variables.at_table)
    ]

    print(json.dumps(tables_present))


if __name__ == "__main__":
    main()
