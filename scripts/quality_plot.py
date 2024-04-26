import numpy as np
import matplotlib.pyplot as plt
from typing import Literal

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
            min_ratio = opt_cost/min_cost
            avg_ratio = opt_cost/avg_cost
            max_ratio = opt_cost/max_cost
            ax[row, column].plot(j, min_ratio,  marker="x", c=plot_utils.COLOR_DICT[search_type])
            ax[row, column].errorbar(
                    j, avg_ratio, yerr=np.std(plot_utils.qualities_from_costs(plot_utils.extract_best_costs(instance, search_type), opt_cost)),
                    capsize=2, c=plot_utils.COLOR_DICT[search_type])
            ax[row, column].plot(j, avg_ratio,  marker="o", c=plot_utils.COLOR_DICT[search_type])
            ax[row, column].plot(j, max_ratio, marker="+", c=plot_utils.COLOR_DICT[search_type])
        ax[row, column].grid(axis='y')
        # if instance == "tai60a" or instance == "tai80a":
            # ax[row, column].set(yticks=[0.88, 0.90, 0.92, 0.94, 0.96, 0.98, 1.0])
            # ax[row, column].spines["left"].set_color("")
        #     ax[row, column].spines["left"].set_linewidth(2)
        # else:
        #     ax[row, column].set(yticks=[0.65, 0.70, 0.75, 0.80, 0.85, 0.90, 0.95, 1.0])
        ax[row, column].axhline(opt_cost/plot_utils.HEURISTIC_COSTS[i], linestyle="dashed", c="black")
    plt.setp(ax, xticks=list(range(len(plot_utils.SEARCH_TYPES))),
             xticklabels=[plot_utils.LABELS_SEARCH_TYPES[st]
                          for st in plot_utils.SEARCH_TYPES])
    plt.tight_layout()
    plt.gcf().set_size_inches(12, 6)
    # print(plt.rcParams.keys())
    # print(fig.rcParams.keys())
    plt.subplots_adjust(hspace=0.3)
    plt.subplots_adjust(wspace=0.3)
    if export_pdf:
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
        ax[row, column].grid(axis='y')
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
                # abandoned trick for moving the bars towards the center when fewer search types are used
                # ax[row, column].plot(
                #     0.25+j*0.5, stat_mean,  marker="o",
                #     c=plot_utils.COLOR_DICT[search_type])
            if "max" in reported_agg:
                ax[row, column].plot(
                    j, np.max(stat_values),  marker="+",
                    c=plot_utils.COLOR_DICT[search_type])
            if stat == "runtime" or "evaluated":
                ax[row, column].ticklabel_format(axis='y', style='sci', scilimits=(3,4))
        if stat == "runtime":
            ax[row, column].axhline(plot_utils.HEURISTIC_AVG_RUNNING_TIMES[i], linestyle="dashed", c="black")
    plt.setp(ax, xticks=list(range(len(search_types))),
            xticklabels=[plot_utils.LABELS_SEARCH_TYPES[st]
                        for st in search_types])
    if len(search_types) > 2:
        plt.tight_layout()
    plt.gcf().set_size_inches(12, 6)
    if export_pdf:
        if not export_path:
            export_path = f"{stat}_plot.pdf"
        plt.savefig(export_path, format="pdf")
    if show_plot:
        plt.show()


def efficiency_plot(
        reported_agg: tuple[str] = ("mean"),
        efficiency_mode: Literal["evaluated"] | Literal["visited"] | Literal["running_time"] = "evaluated",
        search_types: list[str] = plot_utils.SEARCH_TYPES,
        include_heuristic: bool = False,
        export_path: str = "", export_pdf: bool = False,
        show_plot: bool = False) -> None:
    n_rows = 2
    n_cols = int(np.ceil(len(plot_utils.INSTANCE_NAMES) / n_rows))
    fig, ax = plt.subplots(nrows=n_rows, ncols=n_cols)
    for i, instance in enumerate(plot_utils.INSTANCE_NAMES):
        row = i // n_cols
        column = i % n_cols
        ax[row, column].set_title(instance)
        ax[row, column].grid(axis='y')
        opt_cost, opt_sol = results_loading.load_optimum(instance)
        for j, search_type in enumerate(search_types):
            efficiency_values = plot_utils.efficiency_values(opt_cost, instance, search_type, efficiency_mode)
            if "min" in reported_agg:
                ax[row, column].plot(
                    j, np.min(efficiency_values),  marker="x",
                    c=plot_utils.COLOR_DICT[search_type])
            if "mean" in reported_agg:
                stat_mean = np.mean(efficiency_values)
                ax[row, column].errorbar(
                    j, stat_mean, yerr=np.std(efficiency_values),
                    capsize=2, c=plot_utils.COLOR_DICT[search_type])
                ax[row, column].plot(
                    j, stat_mean,  marker="o",
                    c=plot_utils.COLOR_DICT[search_type])
                # if search_type != "greedy":
                #     ax[row, column].text(j, 1.5*stat_mean, str(np.round(stat_mean, 3)), horizontalalignment="left", verticalalignment="center", size=12)
                # else:
                #     ax[row, column].text(j, 1.1*stat_mean, str(np.round(stat_mean, 3)), horizontalalignment="left", verticalalignment="center", size=12)
            if "max" in reported_agg:
                ax[row, column].plot(
                    j, np.max(efficiency_values),  marker="+",
                    c=plot_utils.COLOR_DICT[search_type])
            # if efficiency_mode == "running_time":
            #     ax[row, column].axhline(heuristic_quality/plot_utils.HEURISTIC_AVG_RUNNING_TIMES[i], linestyle="dashed", c="black")
        if include_heuristic:
            heuristic_quality = opt_cost/plot_utils.HEURISTIC_COSTS[i]
            # if instance[3:6].isnumeric():
            #     instance_size = float(instance[3:6])
            # else:
            #     instance_size = float(instance[3:5])
            # norm_factor = 0.5 * instance_size * (instance_size-1)
            norm_factor = 1.0
            match efficiency_mode:
                case "running_time":
                    ax[row, column].axhline(norm_factor*heuristic_quality/plot_utils.HEURISTIC_AVG_RUNNING_TIMES[i], linestyle="dashed", c="black")
                # in other cases, heuristic evaluated/visited 0 or 1 solution (depending on the interpretation),
                # so it only makses sense to have the efficiency value equal to raw quality
                case _:
                    ax[row, column].axhline(norm_factor*heuristic_quality, linestyle="dashed", c="black")


    plt.setp(ax, xticks=list(range(len(search_types))),
             xticklabels=[plot_utils.LABELS_SEARCH_TYPES[st]
                          for st in search_types],
        )
    # for row in range(n_rows):
    #     for col in range(n_cols):
    #         ylabs = ax[row,col].yaxis.get_ticklabels()
    #         for label in ylabs[1::2]:
    #             label.set_visible(False)
    # plt.tight_layout()
    # plt.rcParams["hspace"] = 0.5
    # print(plt.rcParams.keys())
    plt.gcf().set_size_inches(12, 6)
    plt.subplots_adjust(hspace=0.3, wspace=0.3)
    if export_pdf:
        if not export_path:
            export_path = f"efficiency_{efficiency_mode}_plot.pdf"
        plt.savefig(export_path, format="pdf")
    if show_plot:
        plt.show()


def heuristic_runtime_plot() -> None:
    def instance_size_from_name(instance_name: str) -> int:
        if instance_name[3:6].isnumeric():
            return int(instance_name[3:6])
        else:
            return int(instance_name[3:5])
    instance_sizes = [instance_size_from_name(instance) for instance in plot_utils.INSTANCE_NAMES]
    runtime_dict = plot_utils.extract_heuristic_running_times()
    average_runtimes = [np.mean(runtimes) for runtimes in runtime_dict.values()]
    # plt.plot(instance_sizes, average_runtimes, linestyle="dashed", marker="o", c="black")
    plt.errorbar(instance_sizes, average_runtimes, [np.std(rt) for rt in runtime_dict.values()], c="black", capsize=2, marker="o", linestyle="dashed")
    plt.xticks(np.arange(10, 101, 10))
    for i, _ in enumerate(plot_utils.INSTANCE_NAMES):
        runtime = average_runtimes[i]
        plt.text(instance_sizes[i]+0.2, runtime-0.3, str(np.round(runtime, 3)), horizontalalignment="left", verticalalignment="top", size=11)
    plt.grid()
    plt.show()


def heuristic_quality_and_runtime() -> None:
    def instance_size_from_name(instance_name: str) -> int:
        if instance_name[3:6].isnumeric():
            return int(instance_name[3:6])
        else:
            return int(instance_name[3:5])
    runtime_dict = plot_utils.extract_heuristic_running_times()
    fig, ax1 = plt.subplots()
    instance_sizes = [instance_size_from_name(instance) for instance in plot_utils.INSTANCE_NAMES]
    color = "black"
    ax1.set_xlabel("Instance size")
    ax1.set_ylabel("Average time", color=color)
    averages = [np.mean(runtimes) for runtimes in runtime_dict.values()]
    ax1.plot(instance_sizes, averages, linestyle="dashed", marker="o", c=color)
    ax1.errorbar(instance_sizes, averages, yerr=[np.std(runtimes) for runtimes in runtime_dict.values()], linestyle="dashed", marker="o", c=color, capsize=2)
    ax1.tick_params(axis='y', labelcolor=color)
    ax1.set_xticks(np.arange(10, 101, 10))
    ax2 = ax1.twinx()  # instantiate a second axes that shares the same x-axis
    color2 = "green"
    opt_costs = [results_loading.load_optimum(instance)[0] for instance in plot_utils.INSTANCE_NAMES]
    qualities = [opt_cost/cost for opt_cost, cost in zip(opt_costs, plot_utils.HEURISTIC_COSTS[:-1])]
    ax2.set_ylabel("Quality", color=color2)  # already handled the x-label with ax1
    ax2.plot(instance_sizes, qualities, linestyle="dashed", marker="o", color=color2)
    ax2.tick_params(axis='y', labelcolor=color2)
    plt.tight_layout()
    plt.show()


if __name__ == "__main__":
    # Quality plot
    quality_plot(show_plot=True, export_pdf=True)

    # Runtime plot
    monitored_stat_plot("runtime", ("mean"), show_plot=True, export_pdf=True)

    # Efficiency plots
    # efficiency_plot(("mean"), "evaluated", show_plot=True, export_pdf=True)
    # efficiency_plot(("mean"), "visited", show_plot=True, export_pdf=True)
    # efficiency_plot(("min", "mean", "max"), "evaluated", show_plot=True)
    # efficiency_plot(efficiency_mode="running_time", export_path="efficiency_running_time.pdf", export_pdf=True, show_plot=True)

    # Average number of algorithm steps (number of visited solutions)
    # monitored_stat_plot("visited", search_types=["greedy", "steepest"], export_path="visited_gs_plot.pdf", export_pdf=True, show_plot=True)
    # monitored_stat_plot("visited", export_pdf=True, show_plot=True)

    # Average number of evaluated solutions
    # monitored_stat_plot("evaluated", ("mean"), export_pdf=True, show_plot=True)

    # Heuristic runtime plots
    # heuristic_runtime_plot()
    # heuristic_quality_and_runtime()
