const NUM_ROWS: usize = 128;
const NUM_COLS: usize = 8;

enum Chunk {
    LO,
    HI,
}

impl From<&str> for Chunk {
    fn from(s: &str) -> Self {
        // blatanly ignoring that this can fail
        if s == "L" || s == "F" {
            Chunk::LO
        } else {
            Chunk::HI
        }
    }
}

fn bin_search(lo: usize, hi: usize, path: Vec<Chunk>) -> usize {
    let mut r = (lo, hi);

    for l in path.iter() {
        let mid = (r.1 - r.0) / 2 + r.0;

        r = match l {
            Chunk::LO => (r.0, mid),
            Chunk::HI => (mid + 1, r.1),
        };
    }

    // due to the data provided and the number of divisions of the rows/cols, r.0 == r.1
    r.0
}

pub fn get_seat_id(seat: &str) -> usize {
    let rows = &seat[..seat.len() - 3];
    let cols = &seat[seat.len() - 3..];

    let row = {
        let path: Vec<Chunk> = rows
            .chars()
            .map(|c| c.to_string().as_str().into())
            .collect();

        bin_search(0, NUM_ROWS - 1, path)
    };

    let col = {
        let path: Vec<Chunk> = cols
            .chars()
            .map(|c| c.to_string().as_str().into())
            .collect();

        bin_search(0, NUM_COLS - 1, path)
    };

    row * 8 + col
}

#[test]
fn test_get_seat_id() {
    let test_cases = [
        ("FBFBBFFRLR", 357), // from description
        ("BFFFBBFRRR", 567),
        ("FFFBBBFRRR", 119),
        ("BBFFBBFRLL", 820),
    ];

    for (seat, id) in test_cases.iter() {
        println!("{}: {}", seat, id);
        let result = get_seat_id(seat);
        assert_eq!(*id, result, "got error on {}", seat);
    }
}
