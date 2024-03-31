import numpy as np
import matplotlib.pyplot as plt

import plot_utils
import results_loading


def quality_init_vs_local_opt(
        instance_names: list[str] = plot_utils.INSTANCE_NAMES,
        search_types: list[str] = ["greedy", "steepest"],
        export_path: str = "init_vs_final_plot.pdf",
        export_pdf: bool = False, show_plot: bool = False) -> None:
    # interpret the spread - low for small, because everyone easily finds opt
    # high for medium, because many local optima?
    n_rows = 2
    n_cols = int(np.ceil(len(instance_names) / n_rows))
    fig, ax = plt.subplots(nrows=n_rows, ncols=n_cols)
    for i, instance in enumerate(instance_names):
        row = i // n_cols
        column = i % n_cols
        opt_cost, opt_sol = results_loading.load_optimum(instance)
        greedy_cor = 0.0
        steepest_cor = 0.0
        for j, search_type in enumerate(search_types):
            initial_costs = plot_utils.extract_initial_costs(instance, search_type)
            local_opt_costs = plot_utils.extract_best_costs(instance, search_type)
            initial_qualities = opt_cost/initial_costs
            local_opt_qualities = opt_cost/local_opt_costs
            ax[row, column].plot(initial_qualities, local_opt_qualities, "o", c=plot_utils.COLOR_DICT[search_type], alpha=0.5, ms=2)
            ax[row, column].set_xlim([np.min(initial_qualities)-0.01, np.max(initial_qualities)+0.01])
            ax[row, column].set_ylim([np.min(local_opt_qualities)-0.01, 1.01])
            if search_type == "greedy":
                greedy_cor = np.round(np.corrcoef(initial_qualities, local_opt_qualities)[0,1],3)
            if search_type == "steepest":
                steepest_cor = np.round(np.corrcoef(initial_qualities, local_opt_qualities)[0,1],3)
        ax[row, column].set_title(f"{instance}\ng_cor={greedy_cor}, s_cor={steepest_cor}")

    # plt.setp(ax, xticks=list(range(2)), xticklabels=["G", "S"])
    plt.tight_layout()
    plt.gcf().set_size_inches(12, 6)
    if export_pdf:
        plt.savefig(export_path, format="pdf")
    if show_plot:
        plt.show()


def quality_init_vs_local_opt_single(
        instance_name: str,
        search_types: list[str] = ["greedy", "steepest"],
        export_path: str = "",
        export_pdf: bool = False, show_plot: bool = False) -> None:
    opt_cost, opt_sol = results_loading.load_optimum(instance_name)
    greedy_cor = 0.0
    steepest_cor = 0.0
    for j, search_type in enumerate(search_types):
        initial_costs = plot_utils.extract_initial_costs(instance_name, search_type)
        local_opt_costs = plot_utils.extract_best_costs(instance_name, search_type)
        initial_qualities = opt_cost/initial_costs
        local_opt_qualities = opt_cost/local_opt_costs
        plt.plot(initial_qualities, local_opt_qualities, "o", c=plot_utils.COLOR_DICT[search_type], alpha=0.5, ms=2)
        plt.xlim([np.min(initial_qualities)-0.01, np.max(initial_qualities)+0.01])
        plt.ylim([np.min(local_opt_qualities)-0.01, 1.01])
        if search_type == "greedy":
            greedy_cor = np.round(np.corrcoef(initial_qualities, local_opt_qualities)[0,1],3)
        if search_type == "steepest":
            steepest_cor = np.round(np.corrcoef(initial_qualities, local_opt_qualities)[0,1],3)
    plt.title(f"{instance_name}\ng_cor={greedy_cor}, s_cor={steepest_cor}")
    plt.tight_layout()
    plt.gcf().set_size_inches(6, 5)
    if export_pdf:
        if not export_path:
            export_path = f"{instance_name}_init_vs_final.pdf"
        plt.savefig(export_path, format="pdf")
    if show_plot:
        plt.show()


def restarts_vs_best_so_far(
        instance_names: list[str] = plot_utils.INSTANCE_NAMES,
        search_types: list[str] = ["greedy", "steepest"],
        export_path: str = "restarts_best_so_far.pdf",
        export_pdf: bool = False, show_plot: bool = False) -> None:
    n_runs = 500
    n_rows = 2
    n_cols = int(np.ceil(len(instance_names) / n_rows))
    fig, ax = plt.subplots(nrows=n_rows, ncols=n_cols)
    for i, instance in enumerate(instance_names):
        row = i // n_cols
        column = i % n_cols
        # opt_cost, opt_sol = results_loading.load_optimum(instance)
        for _, search_type in enumerate(search_types):
            costs = plot_utils.extract_best_costs(instance, search_type)
            # qualities = plot_utils.qualities_from_costs(costs)
            best_costs_so_far = [costs[0]]
            average_costs_so_far = [costs[0]]
            for i in range(1, len(costs)):
                best_costs_so_far.append(np.min(costs[:i+1]))
                average_costs_so_far.append(np.mean(costs[:i+1]))
            ax[row, column].plot(list(range(n_runs)), best_costs_so_far, "x-", c=plot_utils.COLOR_DICT[search_type], ms=2)
            ax[row, column].plot(list(range(n_runs)), average_costs_so_far, "o", c=plot_utils.COLOR_DICT[search_type], alpha=0.5, ms=2)
        ax[row, column].set_xticks([0, 100, 200, 300, 400, 500])
        ax[row, column].set_title(f"{instance}")
        ax[row, column].ticklabel_format(axis='y', style='sci', scilimits=(2,4))
    plt.tight_layout()
    plt.gcf().set_size_inches(12, 6)
    if export_pdf:
        plt.savefig(export_path, format="pdf")
    if show_plot:
        plt.show()


def quality_vs_sol_sim(
        instance_names: list[str] = plot_utils.INSTANCE_NAMES,
        search_types: list[str] = ["greedy", "steepest"],
        export_path: str = "quality_vs_sol_sim.pdf",
        export_pdf: bool = False, show_plot: bool = False) -> None:
    n_rows = 2
    n_cols = int(np.ceil(len(instance_names) / n_rows))
    fig, ax = plt.subplots(nrows=n_rows, ncols=n_cols)
    for i, instance in enumerate(instance_names):
        row = i // n_cols
        column = i % n_cols
        unique_str = ""
        opt_cost, opt_sol = results_loading.load_optimum(instance)
        for j, search_type in enumerate(search_types):
            costs = plot_utils.extract_best_costs(instance, search_type)
            qualities = plot_utils.qualities_from_costs(costs, opt_cost)
            solutions = plot_utils.extract_best_sols(instance, search_type)
            similarities = [plot_utils.solution_similarity(solution, opt_sol) for solution in solutions]
            ax[row, column].plot(qualities/np.max(qualities), similarities, "o", c=plot_utils.COLOR_DICT[search_type], ms=3, alpha=0.3)
            # ax[row, column].plot(list(range(n_runs)), average_costs_so_far, "o", c=plot_utils.COLOR_DICT[search_type], alpha=0.5, ms=2)
            # unique_str += f" {plot_utils.LABELS_SEARCH_TYPES[search_type]}:  {len(np.unique(solutions, axis=0))}"
        ax[row, column].set_title(f"{instance}{unique_str}")
        # ax.set_title(f"{instance}{unique_str}")
    plt.gcf().set_size_inches(12, 6)
    plt.tight_layout()
    if export_pdf:
        plt.savefig(export_path, format="pdf")
    if show_plot:
        plt.show()


def quality_vs_sol_sim_single(
        instance_name: str,
        search_types: list[str] = ["greedy", "steepest"],
        export_path: str = "",
        export_pdf: bool = False, show_plot: bool = False) -> None:
    unique_str = ""
    opt_cost, opt_sol = results_loading.load_optimum(instance_name)
    for _, search_type in enumerate(search_types):
        costs = plot_utils.extract_best_costs(instance_name, search_type)
        qualities = plot_utils.qualities_from_costs(costs, opt_cost)
        solutions = plot_utils.extract_best_sols(instance_name, search_type)
        similarities = [plot_utils.solution_similarity(solution, opt_sol) for solution in solutions]
        plt.plot(qualities/np.max(qualities), similarities, "o", c=plot_utils.COLOR_DICT[search_type], ms=3, alpha=0.3)
    plt.title(f"{instance_name}{unique_str}")
    plt.gcf().set_size_inches(6, 5)
    plt.tight_layout()
    if export_pdf:
        if not export_path:
            export_path = f"{instance_name}_quality_vs_sim.pdf"
        plt.savefig(export_path, format="pdf")
    if show_plot:
        plt.show()


if __name__ == "__main__":
    # quality_init_vs_local_opt(instance_names=["tai256c", "tai60a", "tai80a", "tai100b"],show_plot=True)
    # quality_init_vs_local_opt_single("tai256c", export_pdf=True,show_plot=True)
    # quality_vs_sol_sim_single("tai256c", show_plot=True)
    # quality_init_vs_local_opt(instance_names=["tai10a", "tai20a", "tai30b", "tai40b"],show_plot=True)
    # Quality of the initial solution vs local optimum
    # quality_init_vs_local_opt(export_pdf=True, show_plot=True)
    # Multistart best and average costs over time
    # restarts_vs_best_so_far(export_pdf=True, show_plot=True, export_path="restarts_both.pdf")

    # quality_vs_sol_sim(instance_names=["tai256c", "tai60a", "tai80a", "tai100b"], show_plot=True)
    # quality_vs_sol_sim(instance_names=["tai256c"], show_plot=True, export_pdf=True, export_path="quality_vs_sim_256.pdf")
    # quality_vs_sol_sim(instance_names=["tai40b", "tai50b", "tai100b", "tai256c"], show_plot=True, export_pdf=True, export_path="quality_vs_sim_256.pdf")
    quality_vs_sol_sim_single("tai256c", show_plot=True, export_pdf=True, export_path="quality_vs_sim_tai256c.pdf")
