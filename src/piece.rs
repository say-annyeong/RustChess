use std::{
    fmt::{self, Display, Formatter},
    collections::HashMap,
    sync::Arc,
    any::Any,
};
use rayon::prelude::{ParallelIterator, IntoParallelIterator};
use regex::Regex;
use lazy_static::lazy_static;
use crate::BOARD_X_SIZE;

// 2차원 벡터
pub type VecXY<T> = Vec<Vec<T>>;
pub type Board2D = BoardXD<2>;
pub type Board3D = BoardXD<3>;

lazy_static! {
    static ref PLAYER_INPUT_RE: Regex = Regex::new(
        r"(?P<name>[A-Za-z]*)(?P<start_col>[A-Za-z]*)(?P<start_row>\d*)(?P<takes>[Xx]?)(?P<end_col>[A-Za-z]+)(?P<end_row>\d+)(?P<other>.*)"
    ).unwrap();
}

/// 칸의 기물 정보를 위한 구조체.
#[derive(Clone, Debug, Default)]
pub struct Piece {
    color: Option<String>,
    piece_type: Option<String>,
    other: Option<Vec<String>>
}

impl Piece {
    fn new(color: Option<String>, piece_type: Option<String>, other: Option<Vec<String>>) -> Self {
        Self { color, piece_type, other }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match (&self.color, &self.piece_type, &self.other) {
            (Some(color), None, None) => write!(f, "{}", color),
            (None, Some(piece_type), None) => write!(f, "{}", piece_type),
            (Some(color), Some(piece_type), None) => write!(f, "{}{}", color, piece_type),
            (None, None, Some(other)) => write!(f, "{:?}", other),
            (Some(color), None, Some(other)) => write!(f, "{}{:?}", color, other),
            (None, Some(piece_type), Some(other)) => write!(f, "{}{:?}", piece_type, other),
            (Some(color), Some(piece_type), Some(other)) => write!(f, "{}{}{:?}", color, piece_type, other),
            (None, None, None) => write!(f, "None")
        }
    }
}

/// 보드 저장시 차원의 제한을 헤제하기 위한 구조체.
/// board_size: 보드의 크기.
/// pieces: 특정 칸의 기물의 정보와 기타 정보를 담음.
pub struct BoardXD<const D: usize> {
    board_size: Vec<usize>,
    pieces: HashMap<Vec<usize>, (Piece, Vec<String>)>
}

impl<const D: usize> BoardXD<D> {
    pub fn new(board_size: Vec<usize>) -> Self {
        if board_size.len() != D { panic!("Board{}D is not Board{}D!", D, D) }
        BoardXD { board_size, pieces: HashMap::new() }
    }

    fn dimension() -> usize { D }
}

/// 기물의 움직임 가능성 표현을 위한 구조체.
///
/// 이 구조체는 각 기물의 미래의 이동 가능성을 저장하는 구조체이다.
///
/// 필드 설명:
///
/// - 'cx': 기물이 x축 방향에서 어디서 시작하는가.
/// - 'cy': 기물이 y축 방향에서 어디서 시작하는가.
/// - 'x': 기물이 x축 방향에서 어디로 도착하는가.
/// - 'y': 기물이 y축 방향에서 어디로 도착하는가.
/// - 'move_type': 이동, 캡쳐, 체크 등을 저장하는 이동 타입.
/// - 'color': 기물의 색상.
/// - 'takes_color': 잡은 기물의 색상.
/// - 'takes_piece_type': 잡은 기물의 종류.
/// - 'other': 기물의 추가적인 상태를 정의하는 문자열 목록입니다.
///
/// # 예시:
///
/// ```rust
/// MoveType { 0, 0, 1, 1, "m", "bishop", "white", None, None, ["move", "capture"] }
/// // (0, 0)에서 출발하며, (1, 1)로 이동이 가능하며, 이동하는 속성을 가진다. 기물의 색상과 종류는 백색 비숍이다. 이동과 잡기가 가능하다.
/// ```
///
#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug, Default)]
pub struct MoveType {
    c_positions: Option<Vec<usize>>,
    positions: Option<Vec<usize>>,
    move_type: Option<String>,
    piece_type: Option<String>,
    color: Option<String>,
    takes_color: Option<String>,
    takes_piece_type: Option<String>,
    other: Option<Vec<String>>
}

impl MoveType {
    pub fn new(c_positions: Option<Vec<usize>>, positions: Option<Vec<usize>>, move_type: Option<String>,
               piece_type: Option<String>, color: Option<String>, takes_color: Option<String>,
               takes_piece_type: Option<String>, other: Option<Vec<String>>) -> Self {
        Self { c_positions, positions, move_type, piece_type, color, takes_color, takes_piece_type, other }
    }

    fn all_none(&self) -> bool {
        self.c_positions == None && self.positions == None && self.move_type == None && self.piece_type == None && self.color == None && self.takes_color == None && self.takes_piece_type == None
    }

    fn set_other(input: Option<Vec<String>>) -> Self {
        let mut move_type = Self::default();
        move_type.other = input;
        move_type
    }
}

/// 기물의 이동 정의를 위한 구조체
///
/// 이 구조체는 각 기물의 이동 범위, 이동 횟수, 기물의 색상 및 타입,
/// 그리고 기물이 특정 상태를 나타내는 특성들을 정의합니다.
///
/// 필드 설명:
///
/// - `dx`: 기물이 x축 방향으로 얼마나 움직일지를 정의합니다.
/// - `dy`: 기물이 y축 방향으로 얼마나 움직일지를 정의합니다.
/// - `times`: 기물이 이동을 반복할 횟수입니다. 예를 들어, `times`가 2라면 기물은 같은 방향으로 두 번 이동할 수 있습니다.
/// - `color`: 기물의 색상입니다. 예를 들어, "white" 또는 "black".
/// - `piece_type`: 기물의 종류를 정의합니다. 예를 들어, "pawn", "king", "queen" 등.
/// - `other`: 기물의 추가적인 상태를 정의하는 문자열 목록입니다. 기물에 특정 특성이 있을 때 사용됩니다.
///
/// `other` 필드에 정의 가능한 상태 목록:
///
/// - `move`: 도착할 칸이 비어 있으면 이동 할 수 있습니다.
/// - `capture`: 도착할 칸에 상대방 기물이 있으면 그 기물을 잡고 이동할 수 있습니다.
/// - `check`: 이동 후, 상대 왕에게 'check'을 걸 수 있음을 의미합니다.
/// - `threatened`: 도착할 칸이 적의 기물의 공격 범위 안에 있으면 이동할 수 없습니다.
/// - `checkmate`: 게임이 종료될 수 있는 상황으로, 이 상태에 도달하면 게임이 끝납니다.
/// - `promotion`: 이 기물이 특정 조건을 만족하면 승진할 수 있음을 의미합니다.
///
/// # 예시:
///
/// ```rust
/// let move_definition = WalkType::new(1, 0, 1, "white".to_string(), "pawn".to_string(), vec!["move".to_string(), "promotion".to_string()]);
/// // x는 1, y는 0방향으로 1번 도착이 가능하다. 색상은 흰색이다. 기물 종류는 폰이다. 도착할 칸이 비어 있으면 이동 가능하며, 특정 조건을 만족하면 승진한다.
/// ```
#[derive(Clone, Debug)]
pub struct WalkType {
    d_positions: Vec<isize>,
    times: usize,
    color: String,
    piece_type: String,
    other: Vec<String>
}

impl WalkType {
    fn new(d_positions: Vec<isize>, times: usize, color: String, piece_type: String, other: Vec<String>) -> Self {
        Self { d_positions, times, color, piece_type, other }
    }
}

/// 수 계산을 위한 구조체
///
/// 필드 설명:
/// - board: CalculateMoves가 계산 가능한 현재 board
/// - piece_type: CalculateMoves가 계산 가능한 기물 종류들
/// - piece_direction: CalculateMoves가 계산 사능한 이동 정의들
struct CalculateMoves<'a, const D: usize> {
    board: BoardXD<D>,
    piece_type: &'a Vec<String>,
    piece_direction: &'a Vec<WalkType>,
}

impl<'a, const D: usize> CalculateMoves<'a, D> {
    fn new(board: BoardXD<D>, piece_type: &'a Vec<String>, piece_direction: &'a Vec<WalkType>) -> Self {
        Self { board, piece_type, piece_direction }
    }

    fn step(&self, positions: Vec<usize>, walk_type: WalkType) -> MoveType {
        match &self.board.pieces.contains_key(&positions) {
            false if walk_type.other.contains(&"move".to_string()) => {
                MoveType::new(None, Some(positions), Some("m".to_string()), Some(walk_type.piece_type), Some(walk_type.color), None, None, Some(walk_type.other))
            }
            true => {
                let Some(piece) = &self.board.get(&positions);
                MoveType::new(None, Some(positions),  Some("x".to_string()), Some(walk_type.piece_type), Some(walk_type.color), Some(piece.color.clone()), Some(piece.piece_type.clone()), Some(walk_type.other))
            }
            _ => MoveType::default()
        }
    }

    fn walk(&self, c_positions: Vec<usize>, walk_type: WalkType) -> Vec<MoveType> {
        let mut moves = Vec::new();
        let mut positions = c_positions;
        let jump = 0;
        for _ in 0..walk_type.times {
            let next_positions: Vec<_> = positions.into_iter().zip(walk_type.d_positions.clone()).map(|(x, dx)| x as isize + dx).collect();
            if next_positions.iter().any(|x| x < &0) {
                break
            } else {
                positions = next_positions.into_iter().map(|x| x as usize).collect();
            }
            if c_positions.iter().zip(&positions).any(|(cx, x)| cx != x) {
                let mut moving = self.step(positions.clone(), walk_type.clone());
                match moving.all_none() {
                    true => {
                        if let Some(other) = moving.other {
                            if other.contains(&"jump_1".to_string()) && jump == 0 {
                                continue
                            } else {
                                break
                            }
                        } else {
                            break
                        }
                    },
                    false => {
                        moving.c_positions = Some(c_positions.clone());
                        moves.push(moving.clone());
                    }
                }
            }
        }
        moves
    }

    // 이동 규칙에 맞는 이동을 전부 검사.
    fn piece(self: Arc<Self>, positions: Vec<usize>) -> Vec<MoveType> {
        // std::thread::spawn => into_par_iter()
        // for, if => filter_map()
        // extend() => flatten()
        let mut output = self.piece_direction.clone().into_par_iter().filter_map(|walk_type| {
            if board_color == &walk_type.color && board_piece_type == &walk_type.piece_type {
                Some(self.walk(x, y, walk_type))
            } else {
                None
            }
        }).flatten().collect::<Vec<_>>();
        output.sort();
        output
    }

    fn board_piece_search(self: Arc<Self>) -> Vec<MoveType> {
        (&self.board).pieces.keys().flat_map(|x| {
            let self_clone = Arc::clone(&self);
            self_clone.piece(x.clone())
        }).collect()
    }

    fn search_piece(self: Arc<Self>, deep: usize) -> CanMove {
        let piece_search = self.clone().board_piece_search();
        let mut output = HashMap::new();
        if deep > 0 {
            let buffer = piece_search.into_par_iter().map(|moving| {
                let board = self.piece_moved(moving.clone());
                let cache = Arc::new(Self::new(board, self.piece_type, self.piece_direction));
                (cache.search_piece(deep - 1), moving)
            }).collect::<Vec<_>>();
            for (can_move, moving) in buffer {
                output.insert(moving, Box::new(can_move));
            }
        } else {
            for moving in piece_search {
                let moved_board = self.piece_moved(moving.clone());
                output.insert(moving, Box::new(CanMove::CanMoves((moved_board, HashMap::new()))));
            }
        }
        CanMove::CanMoves((self.board.clone(), output))
    }

    fn piece_moved(&self, move_type: MoveType) -> VecXY<Piece> {
        let mut buffer = self.board.clone();
        if let (Some(cx), Some(cy), Some(x), Some(y)) =
            (move_type.cx, move_type.cy, move_type.x, move_type.y)
        {
            buffer[x][y] = buffer[cx][cy].clone();
            buffer[cx][cy] = Piece::new(None, None, None);
            buffer
        } else {
            buffer
        }
    }
}

pub struct MainCalculate {
    board: VecXY<Piece>,
    piece_type: Vec<String>,
    piece_direction: Vec<WalkType>,
    pub save_moves: CanMove
}

impl MainCalculate {
    pub fn new(board: VecXY<Piece>, piece_type: Vec<String>, piece_direction: Vec<WalkType>) -> Self {
        let save_moves = CanMove::None;
        Self { board, piece_type, piece_direction, save_moves }
    }

    pub fn piece_move(&mut self, move_type: MoveType) {
        if let (Some(cx), Some(cy), Some(x), Some(y)) =
            (move_type.cx, move_type.cy, move_type.x, move_type.y)
        {
            (*self.board)[x][y] = self.board[cx][cy].clone();
            self.board[cx][cy] = Piece::default();
        }
    }

    pub fn piece_moved(&self, move_type: MoveType) -> VecXY<Piece> {
        CalculateMoves::new(self.board.clone(), &self.piece_type, &self.piece_direction).piece_moved(move_type)
    }

    pub fn calculate_move(&mut self, deep: usize) {
        let calculate = Arc::new(CalculateMoves::new(self.board.clone(), &self.piece_type, &self.piece_direction));
        self.save_moves = calculate.search_piece(deep);
    }

    pub fn calculate_moved(&self, deep: usize) -> CanMove {
        let calculate = Arc::new(CalculateMoves::new(self.board.clone(), &self.piece_type, &self.piece_direction));
        calculate.search_piece(deep)
    }

    pub fn continue_calculate_moves(&mut self, insert_can_move: &mut CanMove) {
        todo!("할꺼야")
    }
}

impl Default for MainCalculate {
    fn default() -> Self {
        Self::new(default_board(), default_piece_type(), default_piece_move())
    }
}

/// 수 추적 및 통신을 위한 열거형
///
/// 이 열거형은 게임 상태를 추적하고, 수의 연쇄적 진행을 관리하는 데 사용됩니다.
///
/// 필드 설명:
/// - `CanMoves`: 수 추적의 트리 구조. 이 변형은 가능한 모든 이동들을 추적하는 해시맵을 포함하고 있으며,
///   빈 해시맵을 사용하여 수 추적을 일시적으로 중단할 수 있습니다.
///   이 경우 해시맵에 수 추척 결과를 계속 담음으로써 추척을 계속합니다.
///   추적이 계속 진행될 때마다 새로운 이동들이 추가될 수 있습니다.
///     - 'VecXY<Board>': 현재 보드 상태.
///     - 'HashMap<MoveType, Box<Self>>': 현재 보드 상태에 MoveType이 적용된 상태를 Box<Self>에 담습니다.
/// - `Board`: 수 추적이 명시적으로 종료된 상태를 나타냅니다. 이 변형은 게임 보드 상태를 포함하며,
///   수 추적이 완료되었음을 나타냅니다.
/// - `None`: 기본값을 나타낼 때 사용됩니다. 기본값을 설정할 때 사용됩니다.
#[derive(Clone, Debug, Default)]
pub enum CanMove {
    CanMoves((VecXY<Piece>, HashMap<MoveType, Box<Self>>)),
    Board(VecXY<Piece>),
    #[default] None
}

impl CanMove {
    pub fn as_can_moves(&self) -> Option<&(VecXY<Piece>, HashMap<MoveType, Box<Self>>)> {
        match self {
            Self::CanMoves(moves) => Some(moves),
            _ => None
        }
    }

    pub fn as_board(&self) -> Option<&VecXY<Piece>> {
        match self {
            Self::Board(board) => Some(board),
            _ => None,
        }
    }

    pub fn as_value(&self) -> Option<&dyn Any> {
        match self {
            Self::CanMoves(moves) => Some(moves),
            Self::Board(board) => Some(board),
            _ => None
        }
    }
}

pub fn default_board() -> VecXY<Piece> {
    vec![
        vec![
            Piece::new(Some("black".to_string()), Some("rook".to_string()), Some(vec!["move".to_string(), "capture".to_string()])),
            Piece::new(Some("black".to_string()), Some("knight".to_string()), Some(vec!["move".to_string(), "capture".to_string()])),
            Piece::new(Some("black".to_string()), Some("bishop".to_string()), Some(vec!["move".to_string(), "capture".to_string()])),
            Piece::new(Some("black".to_string()), Some("queen".to_string()), Some(vec!["move".to_string(), "capture".to_string()])),
            Piece::new(Some("black".to_string()), Some("king".to_string()), Some(vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()])),
            Piece::new(Some("black".to_string()), Some("bishop".to_string()), Some(vec!["move".to_string(), "capture".to_string()])),
            Piece::new(Some("black".to_string()), Some("knight".to_string()), Some(vec!["move".to_string(), "capture".to_string()])),
            Piece::new(Some("black".to_string()), Some("rook".to_string()), Some(vec!["move".to_string(), "capture".to_string()]))
        ],
        vec![Piece::new(Some("black".to_string()), Some("pawn".to_string()), Some(vec!["move".to_string(), "capture".to_string(), "promotion".to_string()])); 8],
        vec![Piece::default(); 8],
        vec![Piece::default(); 8],
        vec![Piece::default(); 8],
        vec![Piece::default(); 8],
        vec![Piece::new(Some("white".to_string()), Some("pawn".to_string()), Some(vec!["move".to_string(), "capture".to_string(), "promotion".to_string()])); 8],
        vec![
            Piece::new(Some("white".to_string()), Some("rook".to_string()), Some(vec!["move".to_string(), "capture".to_string()])),
            Piece::new(Some("white".to_string()), Some("knight".to_string()), Some(vec!["move".to_string(), "capture".to_string()])),
            Piece::new(Some("white".to_string()), Some("bishop".to_string()), Some(vec!["move".to_string(), "capture".to_string()])),
            Piece::new(Some("white".to_string()), Some("queen".to_string()), Some(vec!["move".to_string(), "capture".to_string()])),
            Piece::new(Some("white".to_string()), Some("king".to_string()), Some(vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()])),
            Piece::new(Some("white".to_string()), Some("bishop".to_string()), Some(vec!["move".to_string(), "capture".to_string()])),
            Piece::new(Some("white".to_string()), Some("knight".to_string()), Some(vec!["move".to_string(), "capture".to_string()])),
            Piece::new(Some("white".to_string()), Some("rook".to_string()), Some(vec!["move".to_string(), "capture".to_string()]))
        ],
    ]
}

pub fn default_piece_type() -> Vec<String> {
    vec!["pawn".to_string(), "knight".to_string(), "bishop".to_string(), "rook".to_string(), "queen".to_string(), "king".to_string()]
}

pub fn default_piece_move() -> Vec<WalkType> {
    vec![
        WalkType::new(vec![-1, 0], 1, "white".to_string(), "pawn".to_string(), vec!["move".to_string(), "promotion".to_string()]),
        WalkType::new(vec![-1, -1], 1, "white".to_string(), "pawn".to_string(), vec!["capture".to_string(), "promotion".to_string()]),
        WalkType::new(vec![-1, 1], 1, "white".to_string(), "pawn".to_string(), vec!["capture".to_string(), "promotion".to_string()]),

        WalkType::new(vec![1, 0], 1, "black".to_string(), "pawn".to_string(), vec!["move".to_string(), "promotion".to_string()]),
        WalkType::new(vec![1, -1], 1, "black".to_string(), "pawn".to_string(), vec!["capture".to_string(), "promotion".to_string()]),
        WalkType::new(vec![1, 1], 1, "black".to_string(), "pawn".to_string(), vec!["capture".to_string(), "promotion".to_string()]),


        WalkType::new(vec![2, 1], 1, "white".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![2, -1], 1, "white".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![1, -2], 1, "white".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![-1, -2], 1, "white".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![-2, -1], 1, "white".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![-2, 1], 1, "white".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![-1, 2], 1, "white".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![1, 2], 1, "white".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),

        WalkType::new(vec![2, 1], 1, "black".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![2, -1], 1, "black".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![1, -2], 1, "black".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![-1, -2], 1, "black".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![-2, -1], 1, "black".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![-2, 1], 1, "black".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![-1, 2], 1, "black".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![1, 2], 1, "black".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),


        WalkType::new(vec![1, 1], usize::MAX, "white".to_string(), "bishop".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![1, -1], usize::MAX, "white".to_string(), "bishop".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![-1, -1], usize::MAX, "white".to_string(), "bishop".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![-1, 1], usize::MAX, "white".to_string(), "bishop".to_string(), vec!["move".to_string(), "capture".to_string()]),

        WalkType::new(vec![1, 1], usize::MAX, "black".to_string(), "bishop".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![1, -1], usize::MAX, "black".to_string(), "bishop".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![-1, -1], usize::MAX, "black".to_string(), "bishop".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![-1, 1], usize::MAX, "black".to_string(), "bishop".to_string(), vec!["move".to_string(), "capture".to_string()]),


        WalkType::new(vec![1, 0], usize::MAX, "white".to_string(), "rook".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![0, -1], usize::MAX, "white".to_string(), "rook".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![-1, 0], usize::MAX, "white".to_string(), "rook".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![0, 1], usize::MAX, "white".to_string(), "rook".to_string(), vec!["move".to_string(), "capture".to_string()]),

        WalkType::new(vec![1, 0], usize::MAX, "black".to_string(), "rook".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![0, -1], usize::MAX, "black".to_string(), "rook".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![-1, 0], usize::MAX, "black".to_string(), "rook".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![0, 1], usize::MAX, "black".to_string(), "rook".to_string(), vec!["move".to_string(), "capture".to_string()]),


        WalkType::new(vec![1, 1], usize::MAX, "white".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![1, 0], usize::MAX, "white".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![1, -1], usize::MAX, "white".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![0, -1], usize::MAX, "white".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![-1, -1], usize::MAX, "white".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![-1, 0], usize::MAX, "white".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![-1, 1], usize::MAX, "white".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![0, 1], usize::MAX, "white".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),

        WalkType::new(vec![1, 1], usize::MAX, "black".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![1, 0], usize::MAX, "black".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![1, -1], usize::MAX, "black".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![0, -1], usize::MAX, "black".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![-1, -1], usize::MAX, "black".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![-1, 0], usize::MAX, "black".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![-1, 1], usize::MAX, "black".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(vec![0, 1], usize::MAX, "black".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),


        WalkType::new(vec![1, 1], 1, "white".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(vec![1, 0], 1, "white".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(vec![1, -1], 1, "white".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(vec![0, -1], 1, "white".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(vec![-1, -1], 1, "white".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(vec![-1, 0], 1, "white".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(vec![-1, 1], 1, "white".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(vec![0, 1], 1, "white".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),

        WalkType::new(vec![1, 1], 1, "black".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(vec![1, 0], 1, "black".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(vec![1, -1], 1, "black".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(vec![0, -1], 1, "black".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(vec![-1, -1], 1, "black".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(vec![-1, 0], 1, "black".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(vec![-1, 1], 1, "black".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(vec![0, 1], 1, "black".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
    ]
}

pub fn default_setting() -> (VecXY<Piece>, Vec<String>, Vec<WalkType>) {
    (default_board(), default_piece_type(), default_piece_move())
}

fn custom_calculate_moved(board: VecXY<Piece>, piece_type: Vec<String>, piece_direction: Vec<WalkType>, deep: usize) -> CanMove {
    MainCalculate::new(board, piece_type, piece_direction).calculate_moved(deep)
}

fn chess_x_convent(input: String) -> usize {
    let input_parse = input.trim().parse::<usize>().unwrap();
    let index: Vec<usize> = unsafe { (0..BOARD_X_SIZE).rev().collect() };
    index[input_parse - 1]
}

fn chess_y_convent(input: String) -> usize {
    input.chars().enumerate().map(|(radix, num)| (num.to_digit(36).unwrap() - 9) as usize * 26_usize.pow(radix as u32)).sum::<usize>() - 1
}

pub fn check_move(moves: Vec<&MoveType>, player_input: String) -> Option<Vec<MoveType>> {
    if let Some(input) = PLAYER_INPUT_RE.captures(player_input.as_str()) {
        let (mut name, start_col, start_row, takes, end_col, end_row, other) = (input["name"].to_lowercase(), input["start_col"].to_lowercase(), input["start_row"].to_string(), !input["takes"].is_empty(), input["end_col"].to_lowercase(), input["end_row"].to_string(), input["other"].to_lowercase());
        let cx = if start_col.is_empty() { None } else { Some(chess_y_convent(start_col)) };
        let cy = if start_row.is_empty() { None } else { Some(chess_x_convent(start_row)) };
        let x = chess_x_convent(end_row);
        let y = chess_y_convent(end_col);

        if name.is_empty() {
            name = "pawn".to_string();
        }

        let mut can_moves = Vec::new();
        for move_type in moves {
            let name_correct = Some(name.clone()) == move_type.piece_type;
            let start_col_correct = cx == move_type.cx || cx.is_none();
            let start_row_correct = cy == move_type.cy || cy.is_none();
            //let takes_correct = if takes { Some("x".to_string()) } else { None } == move_type.move_type;
            let end_col_correct = Some(x) == move_type.x;
            let end_row_correct = Some(y) == move_type.y;
            if name_correct && start_col_correct && start_row_correct && end_col_correct && end_row_correct {
                can_moves.push(move_type);
            }
        }

        Some(can_moves.into_iter().map(|move_type| move_type.clone()).collect())
    } else {
        Some(vec![MoveType::set_other(Some(vec![player_input]))])
    }
}

fn custom_check_move(board: VecXY<Piece>, piece_type: Vec<String>, piece_move: Vec<WalkType>, player_input: String) -> Option<Vec<MoveType>> {
    check_move(custom_calculate_moved(board, piece_type, piece_move, 1).as_can_moves().unwrap().1.keys().collect(), player_input)
}
