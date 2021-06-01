import pandas as pd
import matplotlib.pyplot as plt
import scipy.stats

def main():
    path1 = "./weddings/lahc-solve_complete-suite.csv"
    path2 = "./weddings/hill-solve_complete-suite.csv"

    df1 = pd.read_csv(path1)
    df2 = pd.read_csv(path2)

    n1 = df1["n_people"]
    n2 = df2["n_people"]

    print(scipy.stats.mannwhitneyu(n1, n2))

    t1 = df1["seconds"]
    t2 = df2["seconds"]

    print(scipy.stats.mannwhitneyu(t1, t2))

if __name__ == "__main__":
    main()
