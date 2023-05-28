enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

enum Side {
    White,
    Black,
}

struct Piece {
    creed: PieceType,
    icon: String,
    color: Side,
    position: Pos,
}

struct Pos {
    row: u8,
    col: u8,
}

struct Board {
    pieces: Vec<Piece>,
}

trait setup {
    fn setup(&mut self);
}

mod test {
    use super::*;

    impl setup for Board {
        fn setup(&mut self) {
            self.pieces.push(Piece {
                creed: PieceType::Pawn,
                icon: "P".to_string(),
                color: PieceColor::White,
                position: Pos { row: 2, col: 1 },
            });
        }
    }
}
