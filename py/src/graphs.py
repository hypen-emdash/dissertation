import pandas as pd
import matplotlib.pyplot as plt

ALL_COLUMNS = [
    "wedding",
    "n_people",
    "n_tables",
    "total_happiness",
    "mean_happiness",
    "median_happiness",
    "min_happiness",
    "max_happiness",
    "n_lonely",
    "seconds",
]

NUMERIC_COLUMNS = [
    "n_people",
    "n_tables",
    "total_happiness",
    "mean_happiness",
    "median_happiness",
    "min_happiness",
    "max_happiness",
    "n_lonely",
    "seconds",
]

METRIC_COLUMNS = [
    "total_happiness",
    "mean_happiness",
    "median_happiness",
    "min_happiness",
    "max_happiness",
    "n_lonely",
    "seconds",
]

DISPLAY_COLUMN_NAMES = {
    "wedding": "wedding",
    "n_people": "no. people",
    "n_tables": "no. tables",
    "total_happiness": "total happiness",
    "mean_happiness": "mean happiness",
    "median_happiness": "median happiness",
    "min_happiness": "min happiness",
    "max_happiness": "max happiness",
    "n_lonely": "no. lonely peolple",
    "seconds": "time (s)",
}

SOLVERS = ["hill-solve", "int-prog", "lahc-solve"]
SUITES = ["complete-suite", "ring-suite", "rand-suite", "tense-suite"]


def main():

    # Time vs size, with all solvers on the same graph, a graph for each suite.
    for suite in SUITES:
        paths = [f"weddings/{solver}_{suite}.csv" for solver in SOLVERS]
        dfs = [pd.read_csv(path) for path in paths]
        scatter_graph(dfs, "n_people", "seconds", SOLVERS, suite)

    # summary quality vs size, a graph for each
    # for suite in SUITES:
    #     for solver in SOLVERS:
    #         df = pd.read_csv(get_csv_path(solver, suite))
    #         df = average_by_size(df)
    #         summary(df, solver, suite)
    
    # n_lonely graphs, all solvers on one graph, one graph for each suite
    # for suite in SUITES:
    #     paths = [get_csv_path(solver, suite) for solver in SOLVERS]
    #     dfs = [average_by_size(pd.read_csv(path)) for path in paths]
    #     line_graph(dfs, "n_people", "n_lonely", SOLVERS, suite)
    



def get_csv_path(solver, suite):
    return f"weddings/{solver}_{suite}.csv"

def average_by_wedding(df):
    grouped = df.groupby("wedding")

    averaged_df = pd.DataFrame(columns=ALL_COLUMNS)
    for wedding, group in grouped:
        means = {col_name: [group[col_name].mean()] for col_name in NUMERIC_COLUMNS}
        averaged_row = pd.DataFrame(data=dict({"wedding": [wedding]}, **means))
        averaged_df = pd.concat([averaged_df, averaged_row])

    return averaged_df


def average_by_size(df):
    # We're combining multiple weddings, so it doesn't make sense to keep
    # the filename anymore.
    df = df.drop("wedding", inplace=False, axis=1)

    grouped = df.groupby("n_people")

    averaged_df = pd.DataFrame(columns=NUMERIC_COLUMNS)
    for n_people, group in grouped:
        means = {col_name: [group[col_name].mean()] for col_name in NUMERIC_COLUMNS}
        averaged_row = pd.DataFrame(data=means)
        averaged_df = pd.concat([averaged_df, averaged_row])

    return averaged_df


def scatter_graph(dataframes, x_col, y_col, labels, title, log_y=False):
    styles = [""]
    fig, ax = plt.subplots()
    for df, name in zip(dataframes, labels):
        ax.scatter(df[x_col], df[y_col], marker=".", label=name)

    ax.set_xlabel(DISPLAY_COLUMN_NAMES[x_col])
    ax.set_ylabel(DISPLAY_COLUMN_NAMES[y_col])
    ax.set_title(title)
    ax.legend()

    if log_y:
        ax.semilogy()
    plt.show()


def line_graph(dataframes, x_col, y_col, labels, title, log_y=False):
    fig, ax = plt.subplots()
    for df, name in zip(dataframes, labels):
        ax.plot(df[x_col], df[y_col], marker=".", label=name)

    ax.set_xlabel(DISPLAY_COLUMN_NAMES[x_col])
    ax.set_ylabel(DISPLAY_COLUMN_NAMES[y_col])
    ax.set_title(title)
    ax.legend()

    if log_y:
        ax.semilogy()
    plt.show()


def summary(df, solver, suite):
    fig, ax = plt.subplots()

    ax.plot(df["n_people"], df["max_happiness"])
    ax.plot(df["n_people"], df["mean_happiness"])
    ax.plot(df["n_people"], df["median_happiness"])
    ax.plot(df["n_people"], df["min_happiness"])

    ax.scatter(df["n_people"], df["max_happiness"], label="max", marker=".")
    ax.scatter(df["n_people"], df["mean_happiness"], label="mean", marker="+")
    ax.scatter(df["n_people"], df["median_happiness"], label="median", marker="x")
    ax.scatter(df["n_people"], df["min_happiness"], label="min", marker=".")

    ax.set_xlabel(DISPLAY_COLUMN_NAMES["n_people"])
    ax.set_ylabel("Happiness")
    ax.set_title(f"{solver}, {suite}")
    ax.legend()
    plt.show()

if __name__ == "__main__":
    main()
