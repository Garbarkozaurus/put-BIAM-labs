import numpy as np
import matplotlib.pyplot as plt

import plot_utils
import results_loading


def quality_plot(export_path: str = "quality_plot.pdf",
                 export_pdf: bool = False, show_plot: bool = False) -> None:
    n_rows = 2
    n_cols = int(np.ceil(len(plot_utils.INSTANCE_NAMES) / n_rows))
    fig, ax = plt.subplots(nrows=2, ncols=n_cols)
    for i, instance in enumerate(plot_utils.INSTANCE_NAMES):
        row = i // n_cols
        column = i % n_cols
        ax[row, column].set_title(instance)
        opt_cost, opt_sol = results_loading.load_optimum(instance)
        for j, search_type in enumerate(plot_utils.SEARCH_TYPES):
            min_cost, avg_cost, max_cost = plot_utils.worst_avg_best_cost_instance_search(instance, search_type)
            min_ratio = min_cost/opt_cost
            avg_ratio = avg_cost/opt_cost
            max_ratio = max_cost/opt_cost
            ax[row, column].plot(j, min_ratio,  marker="x", c=plot_utils.COLOR_DICT[search_type])
            ax[row, column].plot(j, avg_ratio,  marker="o", c=plot_utils.COLOR_DICT[search_type])
            ax[row, column].plot(j, max_ratio, marker="+", c=plot_utils.COLOR_DICT[search_type])
        if instance != "tai60a" and instance != "tai80a":
            ax[row, column].set(yticks=[1.0, 1.1, 1.2, 1.3, 1.4, 1.5])
        else:
            ax[row, column].set(yticks=[1.0, 1.04, 1.08, 1.12, 1.16, 1.20])
    plt.setp(ax, xticks=list(range(4)), xticklabels=["R", "RW", "G", "S"])
    plt.tight_layout()
    if export_pdf:
        plt.gcf().set_size_inches(12, 6)
        plt.savefig(export_path, format="pdf")
    if show_plot:
        plt.show()


# visited, eval, runtime
def monitored_stat_plot(
        stat: str, reported_agg: tuple[str] = ("mean"),
        search_types: list[str] = plot_utils.SEARCH_TYPES,
        export_path: str = "", export_pdf: bool = False,
        show_plot: bool = False) -> None:
    # column 0 stores run_id, column 4 - the best solution found
    stat_dict = {"visited": 1, "evaluated": 2, "runtime": 3}
    n_rows = 2
    n_cols = int(np.ceil(len(plot_utils.INSTANCE_NAMES) / n_rows))
    fig, ax = plt.subplots(nrows=n_rows, ncols=n_cols)
    for i, instance in enumerate(plot_utils.INSTANCE_NAMES):
        row = i // n_cols
        column = i % n_cols
        ax[row, column].set_title(instance)
        for j, search_type in enumerate(search_types):
            file_name = f"../saved_results/{search_type}/{instance}.txt"
            stat_values = plot_utils.extract_monitored_stat_by_column(
                file_name, stat_dict[stat])
            if "min" in reported_agg:
                ax[row, column].plot(
                    j, np.min(stat_values),  marker="x",
                    c=plot_utils.COLOR_DICT[search_type])
            if "mean" in reported_agg:
                stat_mean = np.mean(stat_values)
                ax[row, column].errorbar(
                    j, stat_mean, yerr=np.std(stat_values),
                    capsize=2, c=plot_utils.COLOR_DICT[search_type])
                ax[row, column].plot(
                    j, stat_mean,  marker="o",
                    c=plot_utils.COLOR_DICT[search_type])
            if "max" in reported_agg:
                ax[row, column].plot(
                    j, np.max(stat_values),  marker="+",
                    c=plot_utils.COLOR_DICT[search_type])
    plt.setp(ax, xticks=list(range(len(search_types))),
             xticklabels=[plot_utils.LABELS_SEARCH_TYPES[st]
                          for st in search_types])
    plt.tight_layout()
    if export_pdf:
        plt.gcf().set_size_inches(12, 6)
        if not export_path:
            export_path = f"{stat}_plot.pdf"
        plt.savefig(export_path, format="pdf")
    if show_plot:
        plt.show()


if __name__ == "__main__":
    # quality_plot(export_path="quality_plot_bigger.pdf", show_plot=True)
    monitored_stat_plot("visited", ("min", "mean", "max"),
                        ["greedy", "steepest"], show_plot=True)
