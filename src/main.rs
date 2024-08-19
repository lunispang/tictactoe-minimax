#[derive(Debug, PartialEq, Clone, Copy)]
enum Mark {
    X,
    O,
}

impl Mark {
    fn other(&self) -> Mark {
        match self {
            Mark::O => Mark::X,
            Mark::X => Mark::O,
        }
    }

    fn to_char(self) -> char {
        match self {
            Self::O => 'O',
            Self::X => 'X',
        }
    }

    fn to_value(self) -> i8 {
        match self {
            Self::O => 1,
            Self::X => -1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Board {
    marks: [Option<Mark>; 9],
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Turn(Mark),
    Winner(Mark),
    Tie,
}

const NO_MARK: Option<Mark> = None;
impl Board {
    fn new() -> Self {
        Board {
            marks: [NO_MARK; 9],
        }
    }

    fn print(&self) {
        for i in 0..9 {
            print!(
                "{}",
                match &self.marks[i as usize] {
                    None => ' ', //(b'0' + i) as char,
                    Some(m) => m.to_char(),
                }
            );
            if i % 3 == 2 {
                println!();
            } else {
                print!("|");
            }
        }
    }

    fn get_state(&self) -> State {
        for row in 0..3 {
            let mark = self.marks[row * 3];
            if mark.is_none() {
                continue;
            }
            if self.marks.iter().skip(row * 3).take(3).all(|&m| m == mark) {
                return State::Winner(mark.unwrap());
            }
        }
        for col in 0..3 {
            let mark = self.marks[col];
            if mark.is_none() {
                continue;
            }
            if self
                .marks
                .iter()
                .skip(col)
                .step_by(3)
                .take(3)
                .all(|&m| m == mark)
            {
                return State::Winner(mark.unwrap());
            }
        }
        for diag in 0..2 {
            let mark = &self.marks[diag * 2];
            if mark.is_none() {
                continue;
            }
            if self
                .marks
                .iter()
                .skip(diag * 2)
                .step_by(4 - diag * 2)
                .take(3)
                .all(|m| m == mark)
            {
                return State::Winner(mark.unwrap());
            }
        }

        if self.marks.iter().all(Option::is_some) {
            return State::Tie;
        }

        let r = self.marks.iter().filter(|m| m.is_some()).count();
        if r % 2 == 0 {
            State::Turn(Mark::X)
        } else {
            State::Turn(Mark::O)
        }
    }
    fn place(&mut self, i: usize) -> Option<()> {
        let state = self.get_state();
        let turn = match state {
            State::Turn(m) => m,
            _ => {
                return None;
            }
        };

        if i > 8 {
            return None;
        }

        match self.marks[i] {
            Some(_) => None,
            None => {
                self.marks[i] = Some(turn);
                Some(())
            }
        }
    }
    fn empty(&self) -> Vec<usize> {
        self.marks
            .iter()
            .enumerate()
            .filter(|(_, e)| e.is_none())
            .map(|(i, _)| i)
            .collect()
    }
}

fn minimax(board: Board, player: Mark) -> (usize, i8) {
    let possible = board.empty();
    let mut results = Vec::new();
    for mve in possible {
        let mut new_board = board;
        new_board.place(mve);
        match new_board.get_state() {
            State::Turn(_) => {
                results.push((mve, -minimax(new_board, player.other()).1));
            }
            State::Tie => results.push((mve, 0)),
            State::Winner(m) => results.push((mve, m.to_value() * player.to_value())),
        }
    }
    *results.iter().max_by_key(|t| t.1).unwrap()
}

fn main() {
    let mut board = Board::new();
    while let State::Turn(_) = board.get_state() {
        board.print();
        let mut string = String::new();
        std::io::stdin().read_line(&mut string).unwrap();
        let mve = string.trim().parse();
        match mve {
            Ok(mv) => {
                if board.place(mv).is_none() {
                    println!("piece already placed there/out of bounds");
                    continue;
                }
                if !board.empty().is_empty() {
                    board.place(minimax(board, Mark::O).0).unwrap();
                } else {
                    break;
                }
            }
            Err(_) => {
                println!("invalid position (enter as index in range 0..9)");
                continue;
            }
        };
    }
    board.print();
}
