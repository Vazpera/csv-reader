use std::{error, fs};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// values in the csv file
    pub value_matrix: Vec<Vec<String>>,
    /// cursors pos
    pub cursor_pos: usize,
    /// current string being looked at
    pub current_value: String,
    /// current cell being looked at
    pub current_location: (usize, usize),
    /// is the user editing?
    pub editing: bool,
    /// path to file
    pub path: String,
    /// first row as headers
    pub has_header_row: bool,
    /// first col as headers
    pub has_label_col: bool,
    /// Is graphing
    pub is_graph: bool,
    /// Previous actions
    pub previous_matrices: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            value_matrix: Vec::new(),
            cursor_pos: 0,
            current_value: String::new(),
            current_location: (0, 0),
            editing: false,
            path: String::new(),
            has_header_row: false,
            has_label_col: false,
            is_graph: false,
            previous_matrices: Vec::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(path: String) -> Self {
        let file = std::fs::read_to_string(path.clone()).expect("File read error");
        let value_matrix: Vec<Vec<String>> = file
            .split('\n')
            .map(|x| x.to_string())
            .map(|x: String| x.split(',').map(|x| x.to_string()).collect::<Vec<String>>())
            .collect();

        Self {
            running: true,
            current_value: value_matrix.clone()[0][0].clone(),
            value_matrix: value_matrix.clone(),
            cursor_pos: value_matrix.clone()[0][0].clone().len(),
            current_location: (0, 0),
            editing: false,
            path: path,
            has_header_row: false,
            has_label_col: false,
            is_graph: false,
            previous_matrices: vec![file.clone()],
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
    pub fn update_curr(&mut self) {
        let file = std::fs::read_to_string(self.path.clone()).expect("File read error");
        let value_matrix: Vec<Vec<String>> = file
            .split('\n')
            .map(|x| x.to_string())
            .map(|x: String| x.split(',').map(|x| x.to_string()).collect::<Vec<String>>())
            .collect();
        self.value_matrix = value_matrix;

        self.current_value =
            self.value_matrix[self.current_location.1][self.current_location.0].clone();
        self.cursor_pos = self.current_value.len();
    }

    pub fn move_up(&mut self) {
        match self.current_location.1.checked_sub(1) {
            Some(j) => self.current_location.1 = j,
            None => {}
        };
        self.update_curr();
    }
    pub fn move_down(&mut self) {
        match self.current_location.1 > self.value_matrix.len() - 2 {
            false => self.current_location.1 = self.current_location.1 + 1,
            true => {}
        };
        self.update_curr();
    }
    pub fn move_right(&mut self) {
        match self.current_location.0 > self.value_matrix[0].len() - 2 {
            false => self.current_location.0 = self.current_location.0 + 1,
            true => {}
        };
        self.update_curr();
    }
    pub fn move_left(&mut self) {
        match self.current_location.0.checked_sub(1) {
            Some(j) => self.current_location.0 = j,
            None => {}
        };
        self.update_curr();
    }
    pub fn edit(&mut self, ch: char) {
        self.current_value.insert(self.cursor_pos, ch);
        self.cursor_pos += 1;
    }
    pub fn backspace(&mut self) {
        match self.cursor_pos {
            0 => {}
            _ => {
                self.current_value.remove(self.cursor_pos - 1);
                self.cursor_pos -= 1;
            }
        }
    }
    pub fn enter_editing(&mut self) {
        self.save(true);
        if !self.is_graph {
            self.editing = true;
        }
    }
    pub fn exit_editing(&mut self) {
        self.save(false);
        self.update_curr();
        self.editing = false;
    }
    pub fn toggle_header_row(&mut self) {
        self.has_header_row = !self.has_header_row;
    }
    pub fn toggle_label_col(&mut self) {
        self.has_label_col = !self.has_label_col;
    }
    pub fn save(&mut self, undoable: bool) {
        if undoable == true {
            self.previous_matrices.push(fs::read_to_string(self.path.clone()).unwrap());
        }
        self.value_matrix[self.current_location.1][self.current_location.0] = self.current_value.clone();
        let _ = std::fs::write(
            self.path.clone(),
            self.value_matrix
                .clone()
                .into_iter()
                .enumerate()
                .fold(String::new(), |acc, (j, x)| {
                    let y = x
                        .into_iter()
                        .enumerate()
                        .fold(String::new(), |acc, (i, x)| {
                            if acc.is_empty() {
                                if (i, j) == self.current_location && !undoable {
                                    if self.current_value.is_empty() {
                                        format!(" ")
                                    } else {
                                        format!("{}", self.current_value)
                                    }
                                } else {
                                    format!("{}", x)
                                }
                            } else {
                                if (i, j) == self.current_location {
                                    if self.current_value.is_empty() {
                                        format!("{},", acc)
                                    } else {
                                        format!("{},{}", acc, self.current_value)
                                    }
                                } else {
                                    format!("{},{}", acc, x)
                                }
                            }
                        });
                    if acc.is_empty() {
                        format!("{y}")
                    } else {
                        format!("{acc}\n{y}")
                    }
                }),
        );
    }
    pub fn add_row(&mut self) {
        self.save(true);
        self.value_matrix
            .push(vec![" ".to_string(); self.value_matrix[0].len()]);
    }
    pub fn remove_row(&mut self) {
        if self.value_matrix.len() != 0 {
            self.current_location = (0, 0);
            self.save(true);
            self.value_matrix.pop();
        }
    }
    pub fn add_col(&mut self) {
        self.save(true);
        for row in &mut self.value_matrix {
            row.push(String::new());
        }
    }
    pub fn remove_col(&mut self) {
        if self.value_matrix[0].len() != 0 {
            self.current_location = (0, 0);
            self.save(true);
            for row in &mut self.value_matrix {
                row.pop();
            }
        }
    }
    pub fn toggle_graph_mode(&mut self) {
        self.is_graph = !self.is_graph;
    }
    pub fn undo(&mut self) {
        if let Some(j) = self.previous_matrices.pop() {
            fs::write(self.path.clone(), j);
        }
        self.update_curr();
        self.save(false);
    }
}
