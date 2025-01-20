use std::{
    fmt::{
        self,
        Display,
        Formatter
    },
    collections::HashMap,
    sync::Arc,
};
use rayon::prelude::*;

// 2차원 벡터
pub type VecXY<T> = Vec<Vec<T>>;

/// 보드 정보를 위한 구조체.
#[derive(Clone, Debug)]
pub struct Board {
    color: Option<String>,
    piece_type: Option<String>,
    // 빈 벡터 던지면 아무튼 None임
    other: Vec<String>
}

impl Board {
    pub fn new(color: Option<String>, piece_type: Option<String>, other: Vec<String>) -> Self {
        Self { color, piece_type, other }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return match (&self.color, &self.piece_type, &self.other.is_empty()) {
            (Some(color), None, true) =>                      write!(f, "{}", color),
            (None, Some(piece_type), true) =>                 write!(f, "{}", piece_type),
            (Some(color), Some(piece_type), true) =>  write!(f, "{}{}", color, piece_type),
            (None, None, false) =>                                    write!(f, "{:?}", self.other),
            (Some(color), None, false) =>                     write!(f, "{}{:?}", color, self.other),
            (None, Some(piece_type), false) =>                write!(f, "{}{:?}", piece_type, self.other),
            (Some(color), Some(piece_type), false) => write!(f, "{}{}{:?}", color, piece_type, self.other),
            _ =>                                                      write!(f, "None")
        }
    }
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
#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub struct MoveType {
    pub cx: Option<usize>,
    pub cy: Option<usize>,
    pub x: Option<usize>,
    pub y: Option<usize>,
    pub move_type: Option<String>,
    pub piece_type: Option<String>,
    pub color: Option<String>,
    pub takes_color: Option<String>,
    pub takes_piece_type: Option<String>,
    // 빈 벡터 던지면 아무튼 None임
    pub other: Vec<String>
}

impl MoveType {
    pub fn new(cx: Option<usize>, cy: Option<usize>, x: Option<usize>, y: Option<usize>,
                      move_type: Option<String>, piece_type: Option<String>, color: Option<String>,
                      takes_color: Option<String>, takes_piece_type: Option<String>, other: Vec<String>) -> Self {
        Self { cx, cy, x, y, move_type, piece_type, color, takes_color, takes_piece_type, other }
    }
    
    pub fn none() -> Self {
        Self { cx: None, cy: None, x: None, y: None, move_type: None, piece_type: None, color: None, takes_color: None, takes_piece_type: None, other: Vec::new() }
    }

    fn all_none(&self) -> bool {
        self.cx == None && self.cy == None && self.x == None && self.y == None && self.move_type == None && self.piece_type == None && self.color == None && self.takes_color == None && self.takes_piece_type == None
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
    dx: isize,
    dy: isize,
    times: usize,
    color: String,
    piece_type: String,
    other: Vec<String>
}

impl WalkType {
    fn new(dx: isize, dy: isize, times: usize, color: String, piece_type: String, other: Vec<String>) -> Self {
        Self { dx, dy, times, color, piece_type, other }
    }
}

/// 수 계산을 위한 구조체
///
/// 필드 설명:
/// - board: CalculateMoves가 계산 가능한 현재 board
/// - piece_type: CalculateMoves가 계산 가능한 기물 종류들
/// - piece_direction: CalculateMoves가 계산 사능한 이동 정의들
struct CalculateMoves<'a> {
    board: VecXY<Board>,
    piece_type: &'a Vec<String>,
    piece_direction: &'a Vec<WalkType>,
}

impl<'a> CalculateMoves<'a> {
    fn new(board: VecXY<Board>, piece_type: &'a Vec<String>, piece_direction: &'a Vec<WalkType>) -> Self {
        Self { board, piece_type, piece_direction }
    }

    fn step(&self, x: usize, y: usize, walk_type: WalkType) -> MoveType {
        if self.board.len() <= x || self.board[0].len() <= y {
            // 보드 범위 초과.
            MoveType::new(None, None, None, None, None, None, None, None, None, Vec::new())
        }
        else {
            match (&self.board[x][y].color, &self.board[x][y].piece_type) {
                // 이동 가능.
                (None, None) if walk_type.other.contains(&"move".to_string()) => {
                    MoveType::new(None, None, Some(x), Some(y), Some("m".to_string()), Some(walk_type.piece_type), Some(walk_type.color), None, None, walk_type.other.clone())
                },
                // 그 위치에 잡을 수 있는 기물 있음.
                (Some(color), Some(piece_type)) if color != &walk_type.color && walk_type.other.contains(&"capture".to_string()) => {
                    MoveType::new(None, None, Some(x), Some(y), Some("x".to_string()), Some(walk_type.piece_type), Some(walk_type.color), Some(color.clone()), Some(piece_type.clone()), walk_type.other.clone())
                }
                // 이동 불가.
                _ => MoveType::new(None, None, None, None, None, None, None, None, None, Vec::new())
            }
        }
    }

    fn walk(&self, cx: usize, cy: usize, walk_type: WalkType) -> Vec<MoveType> {
        let mut moves = Vec::new();
        let (mut x, mut y) = (cx, cy);
        let jump = 0;
        for _ in 0..walk_type.times {
            {
                let (next_x, next_y) = (x as isize + walk_type.dx, y as isize + walk_type.dy);
                if next_x < 0 || next_y < 0 {
                    break;
                }
                (x, y) = (next_x as usize, next_y as usize);
            }
            if self.board.len() <= x || self.board[0].len() <= y {
                break;
            }
            if cx != x || cy != y {
                let mut moving = self.step(x, y, walk_type.clone());
                match moving.all_none() {
                    true if moving.other.contains(&"jump_1".to_string()) && jump == 0 => continue,
                    true => break,
                    false => {
                        (moving.cx, moving.cy) = (Some(cx), Some(cy));
                        moves.push(moving.clone());
                    }
                }
            }
        }
        moves
    }

    fn piece(self: Arc<Self>, x: usize, y: usize) -> Vec<MoveType> {
        // 이동 규칙에 맞는 이동을 전부 검사.
        let Some(board_color) = &self.board[x][y].color else {
            return Vec::new();
        };
        let Some(board_piece_type) = &self.board[x][y].piece_type else {
            return Vec::new();
        };
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
        (0..self.board.len()).into_par_iter().flat_map(|x| {
            let self_clone = Arc::clone(&self);
            (0..self.board[0].len()).into_par_iter().flat_map(move |y| {
                let self_clone_clone = Arc::clone(&self_clone);
                self_clone_clone.piece(x, y)
            })
        }).collect::<Vec<_>>()
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
    
    pub fn piece_moved(&self, move_type: MoveType) -> VecXY<Board> {
        let mut buffer = self.board.clone();
        if let (Some(cx), Some(cy), Some(x), Some(y)) =
            (move_type.cx, move_type.cy, move_type.x, move_type.y)
        {
            buffer[x][y] = buffer[cx][cy].clone();
            buffer[cx][cy] = Board::new(None, None, Vec::new());
            buffer
        } else {
            buffer
        }
    }
}

pub struct MainCalculate {
    board: VecXY<Board>,
    piece_type: Vec<String>,
    piece_direction: Vec<WalkType>,
    pub save_moves: CanMove
}

impl MainCalculate {
    pub fn new(board: VecXY<Board>, piece_type: Vec<String>, piece_direction: Vec<WalkType>) -> Self {
        let save_moves = CanMove::None;
        Self { board, piece_type, piece_direction, save_moves }
    }

    pub fn piece_move(&mut self, move_type: MoveType) {
        if let (Some(cx), Some(cy), Some(x), Some(y)) =
            (move_type.cx, move_type.cy, move_type.x, move_type.y)
        {
            (*self.board)[x][y] = self.board[cx][cy].clone();
            self.board[cx][cy] = Board::new(None, None, Vec::new());
        }
    }

    pub fn piece_moved(&self, move_type: MoveType) -> VecXY<Board> {
        CalculateMoves::new(self.board.clone(), &self.piece_type, &self.piece_direction).piece_moved(move_type)
    }

    pub fn calculate_moves(&mut self, deep: usize) {
        let calculate = Arc::new(CalculateMoves::new(self.board.clone(), &self.piece_type, &self.piece_direction));
        self.save_moves = calculate.search_piece(deep);
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

/// 수 추적을 위한 열거형
///
/// 필드 설명:
/// - CanMoves: 수 추적 연쇄형. 빈 해시맵으로 추적 일시 중단 가능.
/// - Board: 수 추적 명시적 종료.
/// - None: 임시 종료 필드
#[derive(Clone, Debug)]
pub enum CanMove {
    CanMoves((VecXY<Board>, HashMap<MoveType, Box<CanMove>>)),
    Board(VecXY<Board>),
    None
}

impl CanMove {
    pub fn as_board(&self) -> Option<&VecXY<Board>> {
        match self {
            CanMove::Board(board) => Some(board),
            _ => None,
        }
    }

    pub fn as_can_moves(&self) -> Option<&(VecXY<Board>, HashMap<MoveType, Box<CanMove>>)> {
        match self {
            CanMove::CanMoves(move_type) => Some(move_type),
            _ => None
        }
    }
}

pub fn default_board() -> VecXY<Board> {
    vec![
        vec![
            Board::new(Some("black".to_string()), Some("rook".to_string()), vec!["move".to_string(), "capture".to_string()]),
            Board::new(Some("black".to_string()), Some("knight".to_string()), vec!["move".to_string(), "capture".to_string()]),
            Board::new(Some("black".to_string()), Some("bishop".to_string()), vec!["move".to_string(), "capture".to_string()]),
            Board::new(Some("black".to_string()), Some("queen".to_string()), vec!["move".to_string(), "capture".to_string()]),
            Board::new(Some("black".to_string()), Some("king".to_string()), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
            Board::new(Some("black".to_string()), Some("bishop".to_string()), vec!["move".to_string(), "capture".to_string()]),
            Board::new(Some("black".to_string()), Some("knight".to_string()), vec!["move".to_string(), "capture".to_string()]),
            Board::new(Some("black".to_string()), Some("rook".to_string()), vec!["move".to_string(), "capture".to_string()])
        ],
        vec![Board::new(Some("black".to_string()), Some("pawn".to_string()), vec!["move".to_string(), "capture".to_string(), "promotion".to_string()]); 8],
        vec![Board::new(None, None, Vec::new()); 8],
        vec![Board::new(None, None, Vec::new()); 8],
        vec![Board::new(None, None, Vec::new()); 8],
        vec![Board::new(None, None, Vec::new()); 8],
        vec![Board::new(Some("white".to_string()), Some("pawn".to_string()), vec!["move".to_string(), "capture".to_string(), "promotion".to_string()]); 8],
        vec![
            Board::new(Some("white".to_string()), Some("rook".to_string()), vec!["move".to_string(), "capture".to_string()]),
            Board::new(Some("white".to_string()), Some("knight".to_string()), vec!["move".to_string(), "capture".to_string()]),
            Board::new(Some("white".to_string()), Some("bishop".to_string()), vec!["move".to_string(), "capture".to_string()]),
            Board::new(Some("white".to_string()), Some("queen".to_string()), vec!["move".to_string(), "capture".to_string()]),
            Board::new(Some("white".to_string()), Some("king".to_string()), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
            Board::new(Some("white".to_string()), Some("bishop".to_string()), vec!["move".to_string(), "capture".to_string()]),
            Board::new(Some("white".to_string()), Some("knight".to_string()), vec!["move".to_string(), "capture".to_string()]),
            Board::new(Some("white".to_string()), Some("rook".to_string()), vec!["move".to_string(), "capture".to_string()])
        ],
    ]
}

pub fn default_piece_type() -> Vec<String> {
    vec!["pawn".to_string(), "knight".to_string(), "bishop".to_string(), "rook".to_string(), "queen".to_string(), "king".to_string()]
}

pub fn default_piece_move() -> Vec<WalkType> {
    vec![
        WalkType::new(-1, 0, 1, "white".to_string(), "pawn".to_string(), vec!["move".to_string(), "promotion".to_string()]),
        WalkType::new(-1, -1, 1, "white".to_string(), "pawn".to_string(), vec!["capture".to_string(), "promotion".to_string()]),
        WalkType::new(-1, 1, 1, "white".to_string(), "pawn".to_string(), vec!["capture".to_string(), "promotion".to_string()]),

        WalkType::new(1, 0, 1, "black".to_string(), "pawn".to_string(), vec!["move".to_string(), "promotion".to_string()]),
        WalkType::new(1, -1, 1, "black".to_string(), "pawn".to_string(), vec!["capture".to_string(), "promotion".to_string()]),
        WalkType::new(1, 1, 1, "black".to_string(), "pawn".to_string(), vec!["capture".to_string(), "promotion".to_string()]),


        WalkType::new(2, 1, 1, "white".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(2, -1, 1, "white".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(1, -2, 1, "white".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(-1, -2, 1, "white".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(-2, -1, 1, "white".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(-2, 1, 1, "white".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(-1, 2, 1, "white".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(1, 2, 1, "white".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),

        WalkType::new(2, 1, 1, "black".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(2, -1, 1, "black".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(1, -2, 1, "black".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(-1, -2, 1, "black".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(-2, -1, 1, "black".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(-2, 1, 1, "black".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(-1, 2, 1, "black".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(1, 2, 1, "black".to_string(), "knight".to_string(), vec!["move".to_string(), "capture".to_string()]),


        WalkType::new(1, 1, usize::MAX, "white".to_string(), "bishop".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(1, -1, usize::MAX, "white".to_string(), "bishop".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(-1, -1, usize::MAX, "white".to_string(), "bishop".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(-1, 1, usize::MAX, "white".to_string(), "bishop".to_string(), vec!["move".to_string(), "capture".to_string()]),

        WalkType::new(1, 1, usize::MAX, "black".to_string(), "bishop".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(1, -1, usize::MAX, "black".to_string(), "bishop".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(-1, -1, usize::MAX, "black".to_string(), "bishop".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(-1, 1, usize::MAX, "black".to_string(), "bishop".to_string(), vec!["move".to_string(), "capture".to_string()]),


        WalkType::new(1, 0, usize::MAX, "white".to_string(), "rook".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(0, -1, usize::MAX, "white".to_string(), "rook".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(-1, 0, usize::MAX, "white".to_string(), "rook".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(0, 1, usize::MAX, "white".to_string(), "rook".to_string(), vec!["move".to_string(), "capture".to_string()]),

        WalkType::new(1, 0, usize::MAX, "black".to_string(), "rook".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(0, -1, usize::MAX, "black".to_string(), "rook".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(-1, 0, usize::MAX, "black".to_string(), "rook".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(0, 1, usize::MAX, "black".to_string(), "rook".to_string(), vec!["move".to_string(), "capture".to_string()]),


        WalkType::new(1, 1, usize::MAX, "white".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(1, 0, usize::MAX, "white".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(1, -1, usize::MAX, "white".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(0, -1, usize::MAX, "white".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(-1, -1, usize::MAX, "white".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(-1, 0, usize::MAX, "white".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(-1, 1, usize::MAX, "white".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(0, 1, usize::MAX, "white".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),

        WalkType::new(1, 1, usize::MAX, "black".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(1, 0, usize::MAX, "black".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(1, -1, usize::MAX, "black".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(0, -1, usize::MAX, "black".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(-1, -1, usize::MAX, "black".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(-1, 0, usize::MAX, "black".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(-1, 1, usize::MAX, "black".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),
        WalkType::new(0, 1, usize::MAX, "black".to_string(), "queen".to_string(), vec!["move".to_string(), "capture".to_string()]),


        WalkType::new(1, 1, 1, "white".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(1, 0, 1, "white".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(1, -1, 1, "white".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(0, -1, 1, "white".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(-1, -1, 1, "white".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(-1, 0, 1, "white".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(-1, 1, 1, "white".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(0, 1, 1, "white".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),

        WalkType::new(1, 1, 1, "black".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(1, 0, 1, "black".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(1, -1, 1, "black".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(0, -1, 1, "black".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(-1, -1, 1, "black".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(-1, 0, 1, "black".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(-1, 1, 1, "black".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
        WalkType::new(0, 1, 1, "black".to_string(), "king".to_string(), vec!["move".to_string(), "capture".to_string(), "check".to_string(), "threatened".to_string(), "checkmate".to_string()]),
    ]
}

pub fn default_setting() -> (VecXY<Board>, Vec<String>, Vec<WalkType>) {
    (default_board(), default_piece_type(), default_piece_move())
}
