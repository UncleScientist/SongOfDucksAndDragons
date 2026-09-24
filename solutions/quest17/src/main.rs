use macroquad::prelude::*;

// mod drawing;

#[macroquad::main("Deadline-Driven Development")]
async fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q17_p1.txt");
    let volcano = Volcano::new(&lines);
    println!("Quest 17, Part 1 = {}", volcano.total_sum_at_radius(10));

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q17_p2.txt");
    let volcano = Volcano::new(&lines);
    println!("Quest 17, Part 2 = {}", volcano.max_destruction_product());

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q17_p3.txt");
    let volcano = Volcano::new(&lines);
    volcano.draw().await;
    println!("{}", volcano.find_path());
    // drawing::draw_x(37, 53).await;
}

#[derive(Debug)]
struct Volcano {
    grid: Vec<Vec<u8>>,
    source: (usize, usize), // row, col
    start: (usize, usize),  // row, col
}

impl Volcano {
    fn new(lines: &[String]) -> Self {
        let mut grid = Vec::new();
        let mut source = (0, 0);
        let mut start = (0, 0);

        for line in lines {
            let mut grid_row = Vec::new();
            for ch in line.chars() {
                match ch {
                    '@' => {
                        source = (grid.len(), grid_row.len());
                        grid_row.push(0);
                    }
                    'S' => {
                        start = (grid.len(), grid_row.len());
                        grid_row.push(0);
                    }
                    _ => grid_row.push(ch as u8 - b'0'),
                }
            }
            grid.push(grid_row);
        }

        Volcano {
            grid,
            source,
            start,
        }
    }

    fn total_sum_at_radius(&self, radius: usize) -> usize {
        let mut total = 0;
        for (row, line) in self.grid.iter().enumerate() {
            for (col, amount) in line.iter().enumerate() {
                if self.within(row, col, radius) {
                    total += *amount as usize;
                }
            }
        }
        total
    }

    fn sum_at_radius(&self, radius: usize) -> usize {
        self.total_sum_at_radius(radius) - self.total_sum_at_radius(radius - 1)
    }

    fn within(&self, row: usize, col: usize, radius: usize) -> bool {
        row.abs_diff(self.source.0).pow(2) + col.abs_diff(self.source.1).pow(2) <= radius * radius
    }

    fn max_destruction_product(&self) -> usize {
        let max_radius = self.grid[0].len() - self.source.0;
        let (radius, amount) = (1..max_radius)
            .map(|radius| (radius, self.sum_at_radius(radius)))
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap();
        radius * amount
    }

    async fn draw(&self) {
        let (width, height) = (self.grid[0].len(), self.grid.len());
        let (w, h) = (width as f32, height as f32);

        loop {
            let (sw, sh) = (screen_width(), screen_height());
            let (scaled_w, scaled_h) = (sw / w, sh / h);

            for row in 0..height {
                for col in 0..width {
                    let (rowf, colf) = (row as f32, col as f32);
                    let rscale = rowf / h;
                    let cscale = colf / w;
                    let val = if self.source == (row, col) {
                        0.0
                    } else if self.start == (row, col) {
                        0.25
                    } else {
                        0.5 + 0.5 * self.grid[row][col] as f32 / 9.0
                    };
                    let color = match (rscale > cscale, rscale > 1.0 - cscale) {
                        (true, true) => Color::new(0.0, 0.0, val, 1.0), // bottom
                        (true, false) => Color::new(0.0, val, 0.0, 1.0), // left
                        (false, true) => Color::new(val, 0.0, 0.0, 1.0), // right
                        (false, false) => Color::new(val, val, 0.0, 1.0), // top
                    };
                    draw_rectangle(colf * scaled_w, rowf * scaled_h, scaled_w, scaled_h, color);
                }
            }

            next_frame().await;
        }
    }

    // final answer: time taken * volano radius
    fn find_path(&self) -> usize {
        let endpoint = SearchPoint::end(self.start);
        aoclib::astar(
            &SearchPoint::new(self.start),
            |point: &SearchPoint| self.find_neighbors(point),
            |point: &SearchPoint| self.calc_heuristic(point),
            |point: &SearchPoint| *point == endpoint,
        )
        .unwrap()
        .1
    }

    fn find_neighbors(&self, point: &SearchPoint) -> Vec<(SearchPoint, usize)> {
        let (row, col) = (point.loc.0, point.loc.1);

        let mut neighbors = Vec::new();
        if row > 0
            && let Some(n) = self.generate_neighbor(row - 1, col, point)
        {
            neighbors.push(n);
        }

        if row < self.grid.len() - 1
            && let Some(n) = self.generate_neighbor(row + 1, col, point)
        {
            neighbors.push(n);
        }

        if col > 0
            && let Some(n) = self.generate_neighbor(row, col - 1, point)
        {
            neighbors.push(n);
        }

        if col < self.grid[0].len() - 1
            && let Some(n) = self.generate_neighbor(row, col + 1, point)
        {
            neighbors.push(n);
        }

        neighbors
    }

    fn generate_neighbor(
        &self,
        row: usize,
        col: usize,
        point: &SearchPoint,
    ) -> Option<(SearchPoint, usize)> {
        let cost = self.grid[row][col] as usize;

        // find out if the volcano reached us
        let time_so_far = point.time + cost;
        let radius = time_so_far / 30;
        if self.within(row, col, radius) {
            return None;
        }

        let sector = self.get_sector(row, col);
        if sector == point.sector || sector == (point.sector + 1) % 4 {
            Some((point.shift(row, col, cost, sector != 0), cost))
        } else {
            None
        }
    }

    fn get_sector(&self, row: usize, col: usize) -> usize {
        match (row > col, row > self.grid[0].len() - col) {
            (true, true) => 2,
            (true, false) => 3,
            (false, true) => 1,
            (false, false) => 0,
        }
    }

    fn calc_heuristic(&self, point: &SearchPoint) -> usize {
        self.start.0.abs_diff(point.loc.0) + self.start.1.abs_diff(point.loc.1)
    }
}

#[derive(Debug, Default, Hash, Eq, PartialEq, Copy, Clone)]
struct SearchPoint {
    loc: (usize, usize), // row, col
    time: usize,
    sector: usize,
    toured: bool, // true if visited more than one sector
}

impl SearchPoint {
    fn new(loc: (usize, usize)) -> Self {
        Self {
            loc,
            ..Default::default()
        }
    }

    fn end(loc: (usize, usize)) -> Self {
        Self {
            loc,
            toured: true,
            ..Default::default()
        }
    }

    fn shift(&self, row: usize, col: usize, cost: usize, toured: bool) -> Self {
        SearchPoint {
            loc: (row, col),
            sector: self.sector,
            time: self.time + cost,
            toured: self.toured || toured,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let lines = aoclib::read_lines("test-input/part1");
        let volcano = Volcano::new(&lines);
        assert_eq!(1573, volcano.total_sum_at_radius(10));
    }

    #[test]
    fn test_part_2() {
        let lines = aoclib::read_lines("test-input/part2");
        let volcano = Volcano::new(&lines);
        assert_eq!(26, volcano.sum_at_radius(1));
        assert_eq!(49, volcano.sum_at_radius(2));
        assert_eq!(109, volcano.sum_at_radius(3));
        assert_eq!(146, volcano.sum_at_radius(4));
        assert_eq!(218, volcano.sum_at_radius(5));
        assert_eq!(199, volcano.sum_at_radius(6));
    }

    #[test]
    fn test_part_2_max() {
        let lines = aoclib::read_lines("test-input/part2");
        let volcano = Volcano::new(&lines);
        assert_eq!(1090, volcano.max_destruction_product());
    }

    #[test]
    fn test_part_3_1() {
        let lines = aoclib::read_lines("test-input/part3.1");
        let volcano = Volcano::new(&lines);
        assert_eq!(592, volcano.find_path());
    }
}
