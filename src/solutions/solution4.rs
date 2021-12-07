use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::read_string_lines;

pub fn solution4() -> () {
    let input_lines = read_string_lines("src/data/solution4.txt");
    println!("{}", solution4a(&input_lines));
    println!("{}", solution4b(&input_lines));
}

#[derive(Copy, Clone)]
enum Space {
    Unmarked(i32),
    Marked
}
type SpaceRef = Rc<RefCell<Space>>;
type Board = Vec<Vec<SpaceRef>>;

const BOARD_SIZE: usize = 5;

trait Solvable {
    fn is_solved(&self) -> bool;
    fn sum_unmarked(&self) -> i32;
}

impl Solvable for Board {
    fn is_solved(&self) -> bool {
        // Check rows
        self.iter()
            .any(|row| row.iter()
                .all(|space| matches!(*space.borrow(), Space::Marked))
            )

        // Check columns
        || (0..BOARD_SIZE)
            .any(|col_idx| self.iter()
                .all(|row| matches!(*row[col_idx].borrow(), Space::Marked))
            )
    }

    fn sum_unmarked(&self) -> i32 {
        self.iter()
            .flat_map(|row| row.iter())
            .map(|space| match *space.borrow() {
                Space::Unmarked(value) => value,
                Space::Marked => 0
            })
            .sum()
    }
}

fn solution4a(input_lines: &[String]) -> i32 {
    let (boards, mut spaces_for_draw, draws) = parse_bingo_input(input_lines);

    for draw in draws {   
        mark_drawn_spaces(&mut spaces_for_draw, draw);

        match boards.iter().find(|board| board.is_solved()) {
            Some(solved_board) => {
                return draw * solved_board.sum_unmarked()
            },
            None => ()
        }
    }

    panic!("No bingo board was solved!");
}

fn solution4b(input_lines: &[String]) -> i32 {
    let (mut boards, mut spaces_for_draw, draws) = parse_bingo_input(input_lines);

    for draw in draws {   
        mark_drawn_spaces(&mut spaces_for_draw, draw);

        assert!(boards.len() != 0, "All boards were eliminated!");

        // Eliminate solved boards
        if boards.len() > 1 {
            boards.retain(|board| !board.is_solved());
        } else {
            // When one board is left, wait for it to complete
            let last_board = &boards[0];
            if last_board.is_solved() {
                return draw * last_board.sum_unmarked()
            }
        }
    }

    panic!("Multiple unsolved boards still remain!");
}

fn mark_drawn_spaces(spaces_for_draw: &mut HashMap<i32, Vec<SpaceRef>>, draw: i32) -> () {  
    if let Some(space_list) = spaces_for_draw.get_mut(&draw) {
        for space in space_list {
            *(space.borrow_mut()) = Space::Marked;
        }
    }
}

fn parse_bingo_input(lines: &[String]) -> (Vec<Board>, HashMap<i32, Vec<SpaceRef>>, Vec<i32>) {
    let mut lines_iter = lines.iter();
    
    // The first line is the list of draws
    let draws = lines_iter
        .next().unwrap() // First line
        .split(',') // Split around commas
        .map(|num_str| str::parse::<i32>(num_str).unwrap()) // Convert strs to ints
        .collect::<Vec<i32>>();

    let mut spaces_for_draw = HashMap::<i32, Vec<SpaceRef>>::new();
    let mut boards = Vec::<Board>::new();

    // Check whether to proceed by looking for the blank line before each board
    while let Some(mut bingo_line) = lines_iter.next() {

        let mut new_board: Board = Vec::new();
        for _ in 0..BOARD_SIZE {
            // Fetch the next line
            bingo_line = lines_iter.next().unwrap();

            let mut new_row: Vec<SpaceRef> = Vec::new();
            for num_str in bingo_line.split_whitespace() {

                let num = str::parse::<i32>(num_str).unwrap();
                let space = Rc::new(RefCell::new(Space::Unmarked(num)));

                spaces_for_draw
                    .entry(num) // Index operator [] currently disallows mutable access
                    .or_insert(Vec::new()) // Provide default vector if this is the first use of the key
                    .push(space.clone()); // "clone" used to create an extra new owner for an Rc

                new_row.push(space);
            }
            new_board.push(new_row);
        }
        boards.push(new_board);
    }

    (boards, spaces_for_draw, draws)
}