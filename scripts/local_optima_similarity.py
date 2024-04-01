import numpy as np
import matplotlib.pyplot as plt


import results_loading
import plot_utils
# solution quality vs. its average similarity to not-worse solutions


def plot_local_opt_quality_vs_not_worse_sim(
        instance_names: list[str] = plot_utils.INSTANCE_NAMES,
        search_types: list[str] = ["greedy", "steepest"],
        export_path: str = "local_opt_quality_vs_sim.pdf",
        export_pdf: bool = False, show_plot: bool = False) -> None:
    n_rows = 2
    n_cols = int(np.ceil(len(plot_utils.INSTANCE_NAMES) / n_rows))
    fig, ax = plt.subplots(nrows=n_rows, ncols=n_cols)
    for i, instance in enumerate(instance_names):
        for search_type in search_types:
            row = i // n_cols
            column = i % n_cols
            best_sols = plot_utils.extract_best_sols(instance, search_type)
            best_costs = plot_utils.extract_best_costs(instance, search_type)
            order = np.argsort(best_costs)
            best_sols = best_sols[order]
            best_costs = best_costs[order]
            opt_cost, opt_sol = results_loading.load_optimum(instance)
            best_qualities = plot_utils.qualities_from_costs(best_costs, opt_cost)
            # don't include the best solution to prevent comparing a solution to itself
            for j, sol in enumerate(best_sols[1:]):
                # similarity to not-worse solutions
                similarities_to_not_worse = [plot_utils.solution_similarity(sol, other_sol) for other_sol in best_sols[:j+1]]
                avg_sim = np.mean(similarities_to_not_worse)
                ax[row, column].plot(best_qualities[j+1], avg_sim, c=plot_utils.COLOR_DICT[search_type], marker="o", ms=2, alpha=0.3)
        ax[row, column].set_title(f"{instance}")
    # plt.tight_layout()
    fig.subplots_adjust(wspace=0.3, hspace=0.3)
    fig.set_size_inches(12, 6)
    if export_pdf:
        plt.savefig(export_path, format="pdf")
    if show_plot:
        plt.show()


if __name__ == "__main__":
    plot_local_opt_quality_vs_not_worse_sim(export_path="not_tight_local_opt_quality_vs_sim.pdf", export_pdf=True, show_plot=True)
