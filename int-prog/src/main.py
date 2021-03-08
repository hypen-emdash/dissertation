from ortools.linear_solver import pywraplp


def plan_dinner(guest_relations, n_tables):
    """Create a seating plan to make people happy.

    args:
    * guest_relations: a list of lists signifying relationship between guests i and j.
                       should be square-dimensioned, integer-valued, symmetric, and 0
                       along the diagonal.
    * n_tables: the number of tables. Should evenly divide the number of guests.
    """

    assert len(
        guest_relations % n_tables == 0, "Partial filling of tables not yet supported."
    )

    solver = pywraplp.Solver.CreateSolver("SCIP")

    n_guests = len(guest_relations)
    table_size = int(n_guests / n_tables)

    # For now, our answer is represented as an graph with edges representing "sits next to".
    # There are of course some restrictions on the structure of this graph, eg complete
    # components of limited size.

    # `seated[i][j]` is 1 if guest j is seated at table i, else 0.
    seated = [
        [solver.IntVar(0, 1, f"seated[{t}][{g}]") for g in range(n_guests)]
        for t in range(n_tables)
    ]

    # `adjacent[i][j]`` is 1 if guests i and j are seated at the same table, else 0.
    # This will have to be controlled by a constraint.
    adjacent = [
        [solver.IntVar(0, 1, f"adjacent[{i}][{j}]") for i in range(n_guests)]
        for j in range(n_guests)
    ]
