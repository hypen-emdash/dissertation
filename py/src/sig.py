import pandas as pd
import matplotlib.pyplot as plt
import scipy.stats

METRIC_COLUMNS = [
    "mean_happiness",
    "median_happiness",
    "min_happiness",
    "max_happiness",
    "n_lonely",
    "seconds",
]

SOLVERS = ["hill-solve", "int-prog", "lahc-solve"]
SUITES = ["complete-suite", "ring-suite", "rand-suite", "tense-suite"]

def main():
    lows = []
    highs = []
    for suite in SUITES:
        for solver1 in SOLVERS:
            for solver2 in SOLVERS:
                if solver1 < solver2:
                    df1 = pd.read_csv(get_csv_path(solver1, suite))
                    df2 = pd.read_csv(get_csv_path(solver2, suite))
                    for col in METRIC_COLUMNS:
                        p = scipy.stats.mannwhitneyu(df1[col], df2[col]).pvalue
                        if p < 0.05:
                            lows.append((suite, solver1, solver2, col, p))
                        else:
                            highs.append((suite, solver1, solver2, col, p))
    
    print("SIGNIFICANT RESULTS:")
    for l in lows:
        print(l)
    print("----------------------------------------")
    print("INSIGNIFICANT RESULTS:")
    for h in highs:
        print(h)
    
    print(f"n sig: {len(lows)}")
    print(f"n nsig: {len(highs)}")

    

def get_csv_path(solver, suite):
    return f"weddings/{solver}_{suite}.csv"

if __name__ == "__main__":
    main()
