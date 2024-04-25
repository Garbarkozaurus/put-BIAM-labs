import matplotlib.pyplot as plt

import results_loading
import plot_utils


def history_plot(instance_name: str, search_types: list[str] = plot_utils.SEARCH_TYPES) -> None:
    # dicard optimal solution
    opt_cost, _ = results_loading.load_optimum(instance_name)
    for search_type in search_types:
        evals, costs = plot_utils.extract_histories(instance_name, search_type)
        for i in range(100):
            # linestyle='None'
            plt.plot(evals[i], plot_utils.qualities_from_costs(costs[i], opt_cost), c=plot_utils.COLOR_DICT[search_type], marker=".", alpha=0.5, markersize=0.7)
    plt.title(instance_name)
    plt.show()


if __name__ == "__main__":
    history_plot("tai100b")
