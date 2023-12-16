use grid::Grid;
use itertools::Itertools;

pub fn sum_expanded_lengths(s: &str, multiplier: usize) -> usize {
    let grid: Grid<char> = Grid::from_vec(
        s.lines().flat_map(|line| line.chars()).collect(),
        s.lines().next().unwrap().len(),
    );
    let empty_rows: Vec<_> = grid
        .iter_rows()
        .enumerate()
        .filter_map(|(i, mut row)| row.all(|&c| c == '.').then_some(i))
        .collect();

    let empty_cols: Vec<_> = grid
        .iter_cols()
        .enumerate()
        .filter_map(|(i, mut col)| col.all(|&c| c == '.').then_some(i))
        .collect();

    let positions = grid
        .indexed_iter()
        .filter_map(|((y, x), c)| if *c == '#' { Some((y, x)) } else { None })
        .collect::<Vec<_>>();

    positions
        .into_iter()
        .combinations(2)
        .map(|combo| {
            let [from, to] = combo[..] else { unreachable!() };

            let range_y = from.0.min(to.0)..from.0.max(to.0);
            let diff_y = range_y.len()
                + empty_rows.iter().filter(|i| range_y.contains(i)).count() * (multiplier - 1);
            let range_x = from.1.min(to.1)..from.1.max(to.1);
            let diff_x = range_x.len()
                + empty_cols.iter().filter(|i| range_x.contains(i)).count() * (multiplier - 1);
            diff_y + diff_x
        })
        .sum()
}
