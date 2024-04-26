import matplotlib.pyplot as plt
import numpy as np

import results_loading
import plot_utils


def history_plot(instance_name: str, search_types: list[str], n_runs: int = 500) -> None:
    # dicard optimal solution
    opt_cost, _ = results_loading.load_optimum(instance_name)
    all_evals, all_costs = zip(*([plot_utils.extract_histories(instance_name, search_type) for search_type in search_types]))
    order = np.arange(len(search_types))
    for i in range(n_runs):
        np.random.shuffle(order)
        for search_idx in order:
            evals = all_evals[search_idx][i]
            costs = all_costs[search_idx][i]
                # linestyle='None'
            plt.plot(evals, plot_utils.qualities_from_costs(costs, opt_cost), c=plot_utils.COLOR_DICT[plot_utils.SEARCH_TYPES[search_idx]], marker=".", alpha=0.5, markersize=1, linestyle="None")
    plt.title(instance_name)
    plt.grid(axis='y')
    plt.show()


def history_plot_subplots(search_types: list[str], n_runs: int = 500) -> None:
    n_rows = 2
    n_cols = int(np.ceil(len(plot_utils.INSTANCE_NAMES) / n_rows))
    fig, ax = plt.subplots(nrows=2, ncols=n_cols)
    for i, instance in enumerate(plot_utils.INSTANCE_NAMES):
        row = i // n_cols
        column = i % n_cols
        ax[row, column].set_title(instance)
        opt_cost, _ = results_loading.load_optimum(instance)
        order = np.arange(len(search_types))
        all_evals, all_costs = zip(*([plot_utils.extract_histories(instance, search_type) for search_type in search_types]))
        for j in range(n_runs):
            np.random.shuffle(order)
            for search_idx in order:
                evals = all_evals[search_idx][j]
                costs = all_costs[search_idx][j]
                ax[row, column].plot(evals, plot_utils.qualities_from_costs(costs, opt_cost), c=plot_utils.COLOR_DICT[plot_utils.SEARCH_TYPES[search_idx]], marker=".", alpha=0.5, markersize=1, linestyle="None")
        ax[row, column].grid(axis='y')
        # ax[row, column].axhline(opt_cost/plot_utils.HEURISTIC_COSTS[i], c="black", linestyle="dashed")
    plt.tight_layout()
    fig.set_size_inches(12, 6)
    plt.subplots_adjust(hspace=0.3)
    plt.subplots_adjust(wspace=0.3)
    plt.savefig("history_subplots_PNG.png", format="png")
    plt.show()


if __name__ == "__main__":
    # for instance_name in plot_utils.INSTANCE_NAMES:
    #     history_plot(instance_name, plot_utils.SEARCH_TYPES.copy())
    history_plot_subplots(plot_utils.SEARCH_TYPES)
