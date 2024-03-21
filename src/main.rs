use std::collections::HashMap;

#[derive(Debug, Clone)]
enum CellType {
    Text,
    Integer,
    Float,
    None,
}

#[derive(Debug, Clone)]
struct Cell {
    pk: String,
    x: i32,
    y: i32,
    string: Option<String>,
    float: Option<f32>,
    integer: Option<i32>,
    cell_type: CellType,
}

impl Cell {
    fn new(
        pk: String,
        x: i32,
        y: i32,
        string: Option<String>,
        float: Option<f32>,
        integer: Option<i32>,
        cell_type: CellType,
    ) -> Cell {
        Cell {
            pk,
            x,
            y,
            string: string,
            float: float,
            integer: integer,
            cell_type,
        }
    }
}

#[derive(Debug)]
struct Spreadsheet {
    cells: HashMap<String, Cell>,
}

impl Spreadsheet {
    fn new() -> Spreadsheet {
        Spreadsheet {
            cells: HashMap::new(),
        }
    }

    fn insert(&mut self, cell: Cell) {
        self.cells.insert(cell.pk.to_string(), cell);
    }

    fn display() -> () {
        todo!();
    }
}

fn main() {
    let mut sheet = Spreadsheet::new();

    let cell = Cell::new(
        String::from("00"),
        0,
        0,
        None,
        None,
        None,
        CellType::Integer,
    );

    sheet.insert(cell);

    dbg!(sheet.cells);
}
