__package__ = "src.year2021"

from ..functions import assert_solution_module_expected_output

from . import (
    day1_1,
    day1_2,
    day2_1,
    day2_2,
    day3_1,
    day3_2,
)


def test_2021():
    for solution_module in (
        day1_1,
        day1_2,
        day2_1,
        day2_2,
        day3_1,
        day3_2,
    ):
        assert_solution_module_expected_output(solution_module)
