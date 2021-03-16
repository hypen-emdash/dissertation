from dataclasses import dataclass
from ortools.linear_solver import pywraplp


@dataclass
class Problem:
    guest_relations: int
    n_tables: int


class Solution:
    def __init__(self, solver,  status, variables):
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
    # TODO: implement
    pass


def get_problem():
    return Problem(
        [
            [0, 1, 1, 1, 0, 0, 0, 0],
            [1, 0, 1, 1, 0, 0, 0, 0],
            [1, 1, 0, 1, 0, 0, 0, 0],
            [1, 1, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 1, 1],
            [0, 0, 0, 0, 1, 0, 1, 1],
            [0, 0, 0, 0, 1, 1, 0, 1],
            [0, 0, 0, 0, 1, 1, 1, 0],
        ],
        2,
    )

def get_negative_problem():
    return Problem(
        [
            [0, -1, -1, -1, 0, 0, 0, 0],
            [-1, 0, -1, -1, 0, 0, 0, 0],
            [-1, -1, 0, -1, 0, 0, 0, 0],
            [-1, -1, -1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, -1, -1, -1],
            [0, 0, 0, 0, -1, 0, -1, -1],
            [0, 0, 0, 0, -1, -1, 0, -1],
            [0, 0, 0, 0, -1, -1, -1, 0],
        ],
        2,
    )


def display_sol(solution):
    if solution.status in [pywraplp.Solver.OPTIMAL, pywraplp.Solver.FEASIBLE]:
        for table in solution.variables.at_table:
            print([x.solution_value() for x in table])


if __name__ == "__main__":
    main()
