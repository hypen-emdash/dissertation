from ortools.linear_solver import pywraplp

solver = pywraplp.Solver.CreateSolver("SCIP")
inf = solver.infinity()

x = solver.IntVar(0.0, inf, "x")
y = solver.IntVar(0.0, inf, "y")
s = solver.IntVar(0.0, inf, "x + y")

solver.Add(x <= 10)
solver.Add(y <= 10)
solver.Add(s == x + y)

solver.Maximize(s)

status = solver.Solve()

if status == pywraplp.Solver.OPTIMAL:
    print(f"obj val = {solver.Objective().Value()}") 
    print(f"x = {x.solution_value()}")
    print(f"y = {y.solution_value()}")
