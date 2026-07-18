from argparse import ArgumentParser
from math import log10, sqrt
from pathlib import Path
from re import compile as re_compile

import matplotlib.pyplot as plt

class DataPlotter:
    DATA_FOLDER = Path(__file__).parent / 'card_game_data'
    POSSIBLE_REMAINING_CARDS = tuple(range(53))
    FILE_REGEX = re_compile(r'run-(\d+)-exp-(\d+)-splits-(\d+).csv')

    def __init__(self, run_number: None | int = None, dark: bool = False) -> None:
        self.run_number = run_number
        if dark:
            self.color = 'w'
            self.other_color = 'red'
        else:
            self.color = 'k'
            self.other_color = 'darkred'
        self.read_data()

    def file_sorter(self, file: Path) -> tuple[int | float, int, int]:
        regex_match = self.FILE_REGEX.match(file.name)
        if regex_match is None:
            # if regex does not match, sort file to the bottom of the list
            return (-1, -1, -1)
        if self.run_number is None:
            return (
                int(regex_match.group(2)), # sort first by number of runs
                int(regex_match.group(3)), # then by number of splits
                int(regex_match.group(1)), # then by recentness of file
            )
        run_number_difference = self.run_number - int(regex_match.group(1))
        # mildly disfavour lower run numbers that are as close to the desired run number
        if 0 < run_number_difference:
            run_number_difference = -run_number_difference + 0.5
        # is mostly intended to sort desired run number to the end of the list
        return (
            run_number_difference, # sort first by closeness to desired run number
            int(regex_match.group(2)), # then by number of runs
            int(regex_match.group(3)), # then by number of splits
        )

    def read_data(self) -> None:
        all_data_files = sorted(
            self.DATA_FOLDER.glob('run-*-exp-*-splits-*.csv'),
            key=self.file_sorter
        )
        if len(all_data_files) == 0:
            raise FileNotFoundError(f'could not find data file in {self.DATA_FOLDER.absolute()}')
        data_file = all_data_files[-1]
        lines = data_file.read_text(encoding='utf-8').splitlines()[1:]
        lines_split = (
            line.split(', ')
            for line in lines
        )
        self.data = {
            int(line[0]): tuple(
                int(cards_left_in_hand)
                for cards_left_in_hand in line[1:]
            )
            for line in lines_split
        }
        self.max_runs = max(self.data.keys())
        self.float_data = {
            runs: tuple(
                result / runs
                for result in results
            )
            for runs, results in self.data.items()
        }
        final_float_results = self.float_data[self.max_runs]
        self.log_difference_data = {
            runs: tuple(
                log10(abs(result - final_result)) if final_result != 0 else 0
                for result, final_result in zip(results, final_float_results)
            )
            for runs, results in self.float_data.items()
            if runs != self.max_runs
        }

    def plot_data(
        self,
        cards_left_in_hand: int = 0,
        with_root_2_ish: bool = False,
    ) -> None:
        if not 0 <= cards_left_in_hand <= 52:
            raise ValueError(f'Can only have 0–52 cards left in the hand, not {cards_left_in_hand}')
        final_result = self.float_data[self.max_runs][cards_left_in_hand]
        plt.plot(
            tuple(
                log10(runs)
                for runs in self.log_difference_data.keys()
            ),
            tuple(
                results[cards_left_in_hand]
                for results in self.log_difference_data.values()
            ),
            self.color,
            label=f'to {final_result:.5e}',
        )
        if with_root_2_ish:
            self.plot_win_proportion_root_2_convergence()
            plt.legend()
        plt.title(f'Convergence of the proportion of games that have ended with {cards_left_in_hand} cards left in the hand to {final_result:.5e}')
        plt.xlabel('log10(Number of runs)')
        plt.ylabel('log10(|proportion - final proportion|)')

    def plot_win_proportion_root_2_convergence(self):
        root_2_ish = 1 / (100 * sqrt(2))
        plt.plot(
            tuple(
                log10(runs)
                for runs in self.float_data.keys()
            ),
            tuple(
                log10(abs(results[0] - root_2_ish))
                for results in self.float_data.values()
            ),
            self.other_color,
            label=f'to {root_2_ish:.5e} [1/100sqrt(2)]',
        )
        plt.title(f'Convergence of the proportion of games won on 1/100sqrt(2)')
        plt.xlabel('log10(Number of runs)')
        plt.ylabel('log10(|win proportion - 1/100sqrt(2)|)')

    def show_data(
        self,
        cards_left_in_hand: int = 0,
        with_root_2_ish: bool = False,
        block: bool = True
    ) -> None:
        self.plot_data(
            cards_left_in_hand=cards_left_in_hand,
            with_root_2_ish=with_root_2_ish
        )
        plt.show(block=block)

    def plot_bar(self, log: bool = False) -> None:
        x = tuple(range(0, 53, 2))
        final_estimates = list(self.float_data[self.max_runs])
        if log:
            for i, final_estimate in enumerate(final_estimates):
                if final_estimate != 0:
                    final_estimates[i] = log10(final_estimate)
        heights = tuple(
            final_estimates[i]
            for i in x
        )
        plt.bar(
            x,
            heights,
            color = self.color,
        )
        plt.xticks(x)
        plt.title('Probability distribution of possible game endings')
        plt.xlabel('Number of cards remaining in the hand at the end of the game')
        if log:
            plt.ylabel('log10(Probability of occurring)')
        else:
            plt.ylabel('Probability of occurring')

    def show_bar(self, log: bool = False) -> None:
        self.plot_bar(log=log)
        plt.show()

if __name__ == '__main__':
    parser = ArgumentParser(
        prog='deterministic_card_game',
        description='Show line graph of convergence of game results.',
    )
    def check_number_of_cards(number_of_cards_str: str) -> int:
        try:
            number_of_cards = int(number_of_cards_str)
        except ValueError:
            parser.error(f'{number_of_cards_str!r} is not a valid int')
        if not 0 <= number_of_cards <= 52:
            parser.error(f'{number_of_cards} is not between 0 and 52 inclusive')
        return number_of_cards
    parser.add_argument(
        'number_of_cards',
        nargs='?',
        default=None,
        type=check_number_of_cards,
        help='Number of cards left in the hand at the end of a game.'
    )
    parser.add_argument(
        '-r', '--run-number',
        nargs='?',
        default=None,
        type=int,
        help='Run number of data file to read. Defaults to data file with the most runs and splits.'
    )
    parser.add_argument(
        '-b', '--bar',
        default=False,
        action='store_true',
        help='Plot bar chart of probability of each game ending.'
    )
    parser.add_argument(
        '--bar-log',
        default=False,
        action='store_true',
        help='Plot bar chart of log10(probability of each game ending).'
    )
    parser.add_argument(
        '-d', '--dark',
        default=False,
        action='store_true',
        help='Use dark theme (white on black).'
    )
    parser.add_argument(
        '-v', '--version',
        action='version',
        version='%(prog)s v0.4.1',
    )
    parsed = parser.parse_args()
    if parsed.dark:
        plt.style.use('dark_background')
    dp = DataPlotter(parsed.run_number, dark = parsed.dark)
    if parsed.bar or parsed.bar_log:
        dp.show_bar(parsed.bar_log)
    else:
        if parsed.number_of_cards is None:
            dp.show_data(with_root_2_ish=True)
        else:
            dp.show_data(parsed.number_of_cards)
