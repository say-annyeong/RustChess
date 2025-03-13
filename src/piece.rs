use std::{
    fmt::{self, Display, Formatter},
    collections::{
        HashMap,
        hash_map::Entry,
        BTreeMap
    },
    sync::Arc,
    any::Any,
};
use std::fmt::write;
use rayon::prelude::{ParallelIterator, IntoParallelIterator};
use regex::Regex;
use lazy_static::lazy_static;
use lib::Dimension;
use crate::BOARD_X_SIZE;

pub type Board2D = BoardXD<2>;
pub type MoveType2D = MoveType<2>;
pub type WalkType2D = WalkType<2>;
pub type CalculateMoves2D<'a> = CalculateMoves<'a, 2>;
pub type MainCalculate2D = MainCalculate<2>;
pub type CanMove2D = CanMove<2>;

lazy_static! {
    static ref PLAYER_INPUT_RE: Regex = Regex::new(
        r"(?P<name>[A-Za-z]*)(?P<start_col>[A-Za-z]*)(?P<start_row>\d*)(?P<takes>[Xx]?)(?P<end_col>[A-Za-z]+)(?P<end_row>\d+)(?P<other>.*)"
    ).unwrap();
}

trait Dimension<const D: usize> {
    fn dimensions() -> usize;
}

/// 칸의 기물 정보를 위한 구조체.
#[derive(Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct Piece {
    color: String,
    name: String,
    other: BTreeMap<String, Vec<String>>
}

impl Piece {
    fn new(color: String, piece_type: String, other: BTreeMap<String, Vec<String>>) -> Self {
        Self { color, name: piece_type, other }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut short_names = self.other.get("short_name").cloned().unwrap_or_else(|| vec![self.name.clone()]);
        let mut short_color_names = self.other.get("short_color_name").cloned().unwrap_or_else(|| vec![self.color.clone()]);
        if short_names.len() > 1 {
            short_names.sort();
        }
        if short_color_names.len() > 1 {
            short_color_names.sort();
        }
        let short_name = short_names.last().unwrap();
        let short_color_name = short_color_names.last().unwrap();
        write!(f, "{}{}", short_color_name, short_name)
    }
}

/// 보드 저장시 차원의 제한을 헤제하기 위한 구조체.
/// board_size: 보드의 크기.
/// pieces: 특정 칸의 기물의 정보와 기타 정보를 담음.
/// positions 해당하는 Vec<usize>는 z, y, x 순서.
#[derive(Clone, Debug, Dimension)]
pub struct BoardXD<const D: usize> {
    board_size: Vec<usize>,
    pieces: HashMap<Vec<usize>, (Piece, HashMap<String, Vec<String>>)>
}

impl<const D: usize> BoardXD<D> {
    pub fn new(board_size: Vec<usize>, pieces: HashMap<Vec<usize>, (Piece, HashMap<String, Vec<String>>)>) -> Self {
        let dimensions = board_size.len();
        if dimensions != D { panic!("Board{}D is not Board{}D!", dimensions, D) }
        BoardXD { board_size, pieces }
    }
}

impl Default for Board2D {
    fn default() -> Self {
        default_board()
    }
}

impl Display for Board2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in 0..self.board_size[0] {
            for x in 0..self.board_size[1] {
                let Some((piece, _other)) = self.pieces.get(&vec![y, x]) else {
                    write!(f, " -").unwrap();
                    continue
                };
                write!(f, "{}", piece).unwrap();
            }
            writeln!(f).unwrap();
        }
        write!(f, "")
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
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default, Dimension)]
pub struct MoveType<const D: usize> {
    c_positions: Option<Vec<usize>>,
    positions: Option<Vec<usize>>,
    move_type: Option<String>,
    piece: Option<Piece>,
    takes_piece: Option<Piece>,
    other: Option<BTreeMap<String, Vec<String>>>
}

impl<const D: usize> MoveType<D> {
    pub fn new(c_positions: Option<Vec<usize>>, positions: Option<Vec<usize>>, move_type: Option<String>,
               piece: Option<Piece>, takes_piece: Option<Piece>, other: Option<BTreeMap<String, Vec<String>>>) -> Self {
        Self { c_positions, positions, move_type, piece, takes_piece, other }
    }

    fn all_none_as_except_other(&self) -> bool {
        self.c_positions == None && self.positions == None && self.move_type == None && self.piece == None && self.takes_piece == None
    }

    fn other(input: Option<BTreeMap<String, Vec<String>>>) -> Self {
        let mut move_type = Self::default();
        move_type.other = input;
        move_type
    }

    fn set_other(&mut self, input: Option<BTreeMap<String, Vec<String>>>) {
        self.other = input;
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
#[derive(Clone, Debug, Dimension)]
pub struct WalkType<const D: usize> {
    d_positions: Vec<isize>,
    times: usize,
    other: HashMap<String, Vec<String>>
}

impl<const D: usize> WalkType<D> {
    fn new(d_positions: Vec<isize>, times: usize, other: HashMap<String, Vec<String>>) -> Self {
        Self { d_positions, times, other }
    }
}

/// 수 계산을 위한 구조체
///
/// 필드 설명:
/// - board: CalculateMoves 계산 가능한 현재 board
/// - piece_type: CalculateMoves 계산 가능한 기물 종류들
/// - piece_direction: CalculateMoves 계산 사능한 이동 정의들
#[derive(Dimension)]
struct CalculateMoves<'a, const D: usize> {
    board: BoardXD<D>,
    piece_type: &'a Vec<String>,
    piece_direction: &'a HashMap<Piece, Vec<WalkType<D>>>
}

impl<'a, const D: usize> CalculateMoves<'a, D> {
    fn new(board: BoardXD<D>, piece_type: &'a Vec<String>, piece_direction: &'a HashMap<Piece, Vec<WalkType<D>>>) -> Self {
        Self { board, piece_type, piece_direction }
    }

    fn step(&self, positions: Vec<usize>, walk_type: WalkType<D>) -> MoveType<D> {
        if let Some((piece, _other)) = self.board.pieces.get(&positions) {
            if walk_type.other.contains_key("capture".into()) {
                return MoveType::new(None, Some(positions), Some("x".into()), None, Some(piece.clone()), Some(walk_type.other.into_iter().collect()))
            }
        }

        if walk_type.other.contains_key("move".into()) {
            return MoveType::new(None, Some(positions), Some("m".into()), None, None, Some(walk_type.other.into_iter().collect()))
        }

        MoveType::default()
    }

    fn walk(&self, c_positions: Vec<usize>, piece_walk_types: (Piece, Vec<WalkType<{ D }>>)) -> Vec<MoveType<D>> {
        let (piece, walk_types) = piece_walk_types;
        walk_types.into_par_iter().flat_map(|walk_type| {
            let mut moves = Vec::new();
            let mut positions = c_positions.clone();
            let mut jump = 0;
            for _ in 0..walk_type.times {
                let next_position: Option<Vec<_>> = positions.iter().zip(walk_type.d_positions.iter()).map(|(x, dx)| *x as isize + dx).map(|x| if x < 0 { None } else { Some(x as usize) }).collect();
                let Some(next_positions) = next_position else { break };

                if next_positions.iter().zip(&self.board.board_size).any(|(x, mx)| x >= mx) { break }

                if c_positions.iter().zip(&next_positions).all(|(cx, x)| cx == x) {
                    continue
                }

                let mut moving = self.step(next_positions.clone(), walk_type.clone());
                match moving.all_none_as_except_other() {
                    true => {
                        if let Some(other) = moving.other {
                            let Some(attribute) = other.get(&"attribute".to_string()) else {
                                break
                            };
                            if attribute.contains(&"jump_1".to_string()) && jump == 0 {
                                jump += 1;
                                continue
                            } else {
                                break
                            }
                        } else {
                            break
                        }
                    },
                    false => {
                        moving.piece = Some(piece.clone());
                        moving.c_positions = Some(c_positions.clone());
                        moves.push(moving.clone());
                    }
                }
                positions = next_positions;
            }
            moves
        }).collect()
    }

    // 이동 규칙에 맞는 이동을 전부 검사.
    fn piece(self: Arc<Self>, positions: Vec<usize>) -> Vec<MoveType<D>> {
        let Some((piece, _)) = &self.board.pieces.get(&positions) else {
            return Vec::new()
        };
        let (board_color, board_piece_type) = (&piece.color, &piece.name);
        // std::thread::spawn => into_par_iter()
        // for, if => filter_map()
        // extend() => flatten()
        self.piece_direction.clone().into_par_iter().filter_map(|walk_type| {
            let (piece, _other) = &walk_type;
            let (walk_type_color, walk_type_piece_type) = (&piece.color, &piece.name);
            if board_color == walk_type_color && board_piece_type == walk_type_piece_type {
                Some(self.walk(positions.clone(), walk_type))
            } else {
                None
            }
        }).flatten().collect()
    }

    fn search_piece(self: Arc<Self>, deep: usize) -> CanMove<D> {
        let piece_search: Vec<_> = (&self.board).pieces.keys().flat_map(|x| {
            let self_clone = Arc::clone(&self);
            self_clone.piece(x.clone())
        }).collect();
        let mut output = HashMap::new();
        if deep > 0 {
            let buffer: Vec<_> = piece_search.into_par_iter().map(|moving| {
                let board = self.piece_moved(moving.clone());
                let cache = Arc::new(Self::new(board, self.piece_type, self.piece_direction));
                (cache.search_piece(deep - 1), moving)
            }).collect();
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

    fn piece_moved(&self, move_type: MoveType<D>) -> BoardXD<D> {
        // 현재 보드를 복제하여 수정 가능한 버퍼를 생성합니다.
        let mut buffer = self.board.clone();

        // move_type에서 현재 위치(c_positions)와 새 위치(positions)가 모두 Some일 때만 진행합니다.
        if let (Some(c_positions), Some(positions)) = (move_type.c_positions, move_type.positions) {

            // 모든 말(piece)의 상태 벡터에서 "moving" 상태를 제거하여,
            // 이전에 설정된 이동 표시를 초기화합니다.
            buffer.pieces.iter_mut().for_each(|(_k, (_t, statuses))| {
                for (_k, v) in statuses.iter_mut() {
                    v.retain(|x| x != "moving")
                }
            });

            // 현재 위치(c_positions)에 해당하는 말이 존재하는지 확인합니다.
            if let Some(piece) = buffer.pieces.get(&c_positions) {
                // 해당 말을 복제하여 이동 후 수정할 새로운 인스턴스를 만듭니다.
                let mut piece = piece.clone();
                // 복제한 말의 상태 벡터에 "moving" 상태를 추가합니다.
                piece.1.entry("attributes".to_string()).and_modify(|v| v.push("moving".to_string())).or_insert(vec!["moving".to_string()]);

                // 새 위치(positions)에 이미 말이 있는지 확인합니다.
                // 만약 해당 위치가 이미 점유되어 있으면, 이동을 중단하고 현재 보드를 그대로 반환합니다.
                match buffer.pieces.entry(positions) {
                    Entry::Occupied(_) => return buffer,
                    // 목적지가 비어 있으면, 새 위치에 이동한 말을 삽입합니다.
                    Entry::Vacant(entry) => entry.insert(piece)
                };

                // 현재 위치(c_positions)에서 말을 제거하여 빈 공간으로 만듭니다.
                buffer.pieces.remove_entry(&c_positions);
                // 수정된 보드(버퍼)를 반환합니다.
                buffer
            } else {
                // 만약 현재 위치에 말이 없다면, 아무런 변경 없이 보드를 반환합니다.
                buffer
            }
        } else {
            // move_type에 필요한 위치 정보가 부족하면, 아무런 변경 없이 보드를 반환합니다.
            buffer
        }
    }
}

#[derive(Dimension)]
pub struct MainCalculate<const D: usize> {
    board: BoardXD<D>,
    piece_type: Vec<String>,
    piece_direction: HashMap<Piece, Vec<WalkType<D>>>,
    pub save_moves: CanMove<D>
}

impl<const D: usize> MainCalculate<D> {
    pub fn new(board: BoardXD<D>, piece_type: Vec<String>, piece_direction: HashMap<Piece, Vec<WalkType<D>>>) -> Self {
        let save_moves = CanMove::None;
        Self { board, piece_type, piece_direction, save_moves }
    }

    pub fn piece_move(&mut self, move_type: MoveType<D>) {
        if let (Some(c_positions), Some(positions)) = (move_type.c_positions, move_type.positions) {
            let mut buffer = &mut self.board.pieces;
            if buffer.contains_key(&c_positions) {
                let Some(v_buffer) = buffer.get(&c_positions).cloned() else {
                    return
                };
                buffer.remove(&c_positions);
                buffer.insert(positions, v_buffer.clone().clone());
            }
        }
    }

    pub fn piece_moved(&self, move_type: MoveType<D>) -> BoardXD<D> {
        CalculateMoves::new(self.board.clone(), &self.piece_type, &self.piece_direction).piece_moved(move_type)
    }

    pub fn calculate_move(&mut self, deep: usize) {
        let calculate = Arc::new(CalculateMoves::new(self.board.clone(), &self.piece_type, &self.piece_direction));
        self.save_moves = calculate.search_piece(deep);
    }

    pub fn calculate_moved(&self, deep: usize) -> CanMove<D> {
        let calculate = Arc::new(CalculateMoves::new(self.board.clone(), &self.piece_type, &self.piece_direction));
        calculate.search_piece(deep)
    }

    pub fn continue_calculate_moves(&mut self, insert_can_move: &mut CanMove<D>) {
        todo!("할꺼야")
    }
}

impl Default for MainCalculate<2> {
    fn default() -> Self {
        Self::new(default_board(), default_piece_type(), default_piece_move())
    }
}

#[derive(Dimension)]
struct ParsePlayerInput<const D: usize> {
    moves: Vec<MoveType<D>>
}

impl ParsePlayerInput<2> {
    fn player_input_parse(&self, player_input: String) -> Option<Vec<MoveType2D>> {
        if let Some(input) = PLAYER_INPUT_RE.captures(player_input.as_str()) {
            let (mut name, start_col, start_row, _takes, end_col, end_row, _other) = (input["name"].to_lowercase(), input["start_col"].to_lowercase(), input["start_row"].to_string(), !input["takes"].is_empty(), input["end_col"].to_lowercase(), input["end_row"].to_string(), input["other"].to_lowercase());
            let cx = if start_col.is_empty() { None } else { Some(chess_y_convent(start_col)) };
            let cy = if start_row.is_empty() { None } else { Some(chess_x_convent(start_row)) };
            let x = chess_x_convent(end_row);
            let y = chess_y_convent(end_col);

            let (player_c_positions, player_positions) = (vec![cy, cx], vec![y, x]);

            if name.is_empty() {
                name = "pawn".to_string();
            }

            let mut can_moves = Vec::new();
            macro_rules! parsing_positions {
                ($input:expr, $output:ident) => {
                    let $output: Vec<_> = $input.as_ref().map(|v| v.iter().map(Some).collect()).unwrap_or_default();
                };
            }

            macro_rules! correct_check {
                ($input1:expr, $input2:expr, $output:ident) => {
                    let $output = $input1.iter().zip($input2).all(|(player_position, position| {
                        match position {
                            Some(pos) => player_position == pos,
                            None => true
                        }
                    });
                };
            }

            for move_type in &self.moves {
                let name_correct = move_type.piece.iter().cloned().any(|move_type| move_type.name == name);
                let (c_positions, positions) = (&move_type.c_positions, &move_type.positions);
                parsing_positions!(c_positions, parsing_c_positions);
                parsing_positions!(positions, parsing_positions);

                correct_check!(player_c_positions, parsing_c_positions, c_positions_correct);
                //correct_check!(player_positions, parsing_positions, positions_correct);

                let positions_correct = player_positions.iter().zip(parsing_positions).all(|(player_position, position)| {
                    match position {
                        Some(pos) => player_position == pos,
                        None => true
                    }
                });

                let c_positions_correct = false;
                let positions_correct = false;

                //let takes_correct = if takes { Some("x".to_string()) } else { None } == move_type.move_type;

                if name_correct && c_positions_correct && positions_correct {
                    can_moves.push(move_type);
                }
            }

            Some(can_moves.into_iter().map(|move_type| move_type.clone()).collect())
        } else {
            Some(vec![MoveType::other(Some(BTreeMap::from([("player_input".to_string(), vec![player_input])])))])
        }
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
#[derive(Clone, Debug, Default, Dimension)]
pub enum CanMove<const D: usize> {
    CanMoves((BoardXD<D>, HashMap<MoveType<D>, Box<Self>>)),
    Board(BoardXD<D>),
    #[default] None
}

impl<const D: usize> CanMove<D> {
    pub fn as_can_moves(&self) -> Option<&(BoardXD<D>, HashMap<MoveType<D>, Box<CanMove<D>>>)> {
        match self {
            Self::CanMoves(moves) => Some(moves),
            _ => None
        }
    }

    pub fn as_board(&self) -> Option<&BoardXD<D>> {
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

macro_rules! default_pieces {
    ($white_pawn:ident, $white_knight:ident, $white_bishop:ident, $white_rook:ident, $white_queen:ident, $white_king:ident, $black_pawn:ident, $black_knight:ident, $black_bishop:ident, $black_rook:ident, $black_queen:ident, $black_king:ident) => {
        let $white_pawn = Piece::new("white".to_string(), "pawn".to_string(), BTreeMap::from([("attributes".to_string(), vec!["promotion".to_string()]), ("short_name".to_string(), vec!["P".to_string()]), ("short_color_name".to_string(), vec!["W".to_string()])]));
        let $white_knight = Piece::new("white".to_string(), "knight".to_string(), BTreeMap::from([("short_name".to_string(), vec!["N".to_string()]), ("short_color_name".to_string(), vec!["W".to_string()])]));
        let $white_bishop = Piece::new("white".to_string(), "bishop".to_string(), BTreeMap::from([("short_name".to_string(), vec!["B".to_string()]), ("short_color_name".to_string(), vec!["W".to_string()])]));
        let $white_rook = Piece::new("white".to_string(), "rook".to_string(), BTreeMap::from([("short_name".to_string(), vec!["R".to_string()]), ("short_color_name".to_string(), vec!["W".to_string()])]));
        let $white_queen = Piece::new("white".to_string(), "queen".to_string(), BTreeMap::from([("short_name".to_string(), vec!["Q".to_string()]), ("short_color_name".to_string(), vec!["W".to_string()])]));
        let $white_king = Piece::new("white".to_string(), "king".to_string(), BTreeMap::from([("attributes".to_string(), vec!["check".to_string(), "threatened".to_string(), "checkmate".to_string()]), ("short_name".to_string(), vec!["K".to_string()]), ("short_color_name".to_string(), vec!["W".to_string()])]));

        let $black_pawn = Piece::new("black".to_string(), "pawn".to_string(), BTreeMap::from([("attributes".to_string(), vec!["promotion".to_string()]), ("short_name".to_string(), vec!["P".to_string()]), ("short_color_name".to_string(), vec!["B".to_string()])]));
        let $black_knight = Piece::new("black".to_string(), "knight".to_string(), BTreeMap::from([("short_name".to_string(), vec!["N".to_string()]), ("short_color_name".to_string(), vec!["B".to_string()])]));
        let $black_bishop = Piece::new("black".to_string(), "bishop".to_string(), BTreeMap::from([("short_name".to_string(), vec!["B".to_string()]), ("short_color_name".to_string(), vec!["B".to_string()])]));
        let $black_rook = Piece::new("black".to_string(), "rook".to_string(), BTreeMap::from([("short_name".to_string(), vec!["R".to_string()]), ("short_color_name".to_string(), vec!["B".to_string()])]));
        let $black_queen = Piece::new("black".to_string(), "queen".to_string(), BTreeMap::from([("short_name".to_string(), vec!["Q".to_string()]), ("short_color_name".to_string(), vec!["B".to_string()])]));
        let $black_king = Piece::new("black".to_string(), "king".to_string(), BTreeMap::from([("attributes".to_string(), vec!["check".to_string(), "threatened".to_string(), "checkmate".to_string()]), ("short_name".to_string(), vec!["K".to_string()]), ("short_color_name".to_string(), vec!["B".to_string()])]));
    };
}

pub fn default_board() -> Board2D {
    default_pieces!(white_pawn, white_knight, white_bishop, white_rook, white_queen, white_king, black_pawn, black_knight, black_bishop, black_rook, black_queen, black_king);
    
    Board2D::new(
        vec![8, 8], 
        HashMap::from(
            [
                (vec![0, 0], (white_rook.clone(), HashMap::new())),
                (vec![0, 1], (white_knight.clone(), HashMap::new())),
                (vec![0, 2], (white_bishop.clone(), HashMap::new())),
                (vec![0, 3], (white_queen, HashMap::new())),
                (vec![0, 4], (white_king, HashMap::new())),
                (vec![0, 5], (white_bishop, HashMap::new())),
                (vec![0, 6], (white_knight, HashMap::new())),
                (vec![0, 7], (white_rook, HashMap::new())),
                (vec![1, 0], (white_pawn.clone(), HashMap::new())), (vec![1, 1], (white_pawn.clone(), HashMap::new())), (vec![1, 2], (white_pawn.clone(), HashMap::new())), (vec![1, 3], (white_pawn.clone(), HashMap::new())), (vec![1, 4], (white_pawn.clone(), HashMap::new())), (vec![1, 5], (white_pawn.clone(), HashMap::new())), (vec![1, 6], (white_pawn.clone(), HashMap::new())), (vec![1, 7], (white_pawn, HashMap::new())),
                (vec![6, 0], (black_pawn.clone(), HashMap::new())), (vec![6, 1], (black_pawn.clone(), HashMap::new())), (vec![6, 2], (black_pawn.clone(), HashMap::new())), (vec![6, 3], (black_pawn.clone(), HashMap::new())), (vec![6, 4], (black_pawn.clone(), HashMap::new())), (vec![6, 5], (black_pawn.clone(), HashMap::new())), (vec![6, 6], (black_pawn.clone(), HashMap::new())), (vec![6, 7], (black_pawn, HashMap::new())),
                (vec![7, 0], (black_rook.clone(), HashMap::new())),
                (vec![7, 1], (black_knight.clone(), HashMap::new())),
                (vec![7, 2], (black_bishop.clone(), HashMap::new())),
                (vec![7, 3], (black_queen, HashMap::new())),
                (vec![7, 4], (black_king, HashMap::new())),
                (vec![7, 5], (black_bishop, HashMap::new())),
                (vec![7, 6], (black_knight, HashMap::new())),
                (vec![7, 7], (black_rook, HashMap::new())),
            ]
        )
    )
}

pub fn default_piece_type() -> Vec<String> {
    vec!["pawn".to_string(), "knight".to_string(), "bishop".to_string(), "rook".to_string(), "queen".to_string(), "king".to_string()]
}

pub fn default_piece_move() -> HashMap<Piece, Vec<WalkType2D>> {
    default_pieces!(white_pawn, white_knight, white_bishop, white_rook, white_queen, white_king, black_pawn, black_knight, black_bishop, black_rook, black_queen, black_king);
    let other_move_capture = HashMap::from([("move_type".to_string(), vec!["move".to_string(), "capture".to_string()])]);
    let other_move_capture_threatened = HashMap::from([("move_type".to_string(), vec!["move".to_string(), "capture".to_string(), "threatened".to_string()])]);
    let default_knight_move = vec![
        WalkType::new(vec![2, 1], 1, other_move_capture.clone()),
        WalkType::new(vec![2, -1], 1, other_move_capture.clone()),
        WalkType::new(vec![1, -2], 1, other_move_capture.clone()),
        WalkType::new(vec![-1, -2], 1, other_move_capture.clone()),
        WalkType::new(vec![-2, -1], 1, other_move_capture.clone()),
        WalkType::new(vec![-2, 1], 1, other_move_capture.clone()),
        WalkType::new(vec![-1, 2], 1, other_move_capture.clone()),
        WalkType::new(vec![1, 2], 1, other_move_capture.clone())
    ];
    let default_bishop_move = vec![
        WalkType::new(vec![1, 1], usize::MAX, other_move_capture.clone()),
        WalkType::new(vec![1, -1], usize::MAX, other_move_capture.clone()),
        WalkType::new(vec![-1, -1], usize::MAX, other_move_capture.clone()),
        WalkType::new(vec![-1, 1], usize::MAX, other_move_capture.clone())
    ];
    let default_rook_move = vec![
        WalkType::new(vec![1, 0], usize::MAX, other_move_capture.clone()),
        WalkType::new(vec![0, -1], usize::MAX, other_move_capture.clone()),
        WalkType::new(vec![-1, 0], usize::MAX, other_move_capture.clone()),
        WalkType::new(vec![0, 1], usize::MAX, other_move_capture.clone())
    ];
    let default_queen_move = vec![
        WalkType::new(vec![1, 1], usize::MAX, other_move_capture.clone()),
        WalkType::new(vec![1, 0], usize::MAX, other_move_capture.clone()),
        WalkType::new(vec![1, -1], usize::MAX, other_move_capture.clone()),
        WalkType::new(vec![0, -1], usize::MAX, other_move_capture.clone()),
        WalkType::new(vec![-1, -1], usize::MAX, other_move_capture.clone()),
        WalkType::new(vec![-1, 0], usize::MAX, other_move_capture.clone()),
        WalkType::new(vec![-1, 1], usize::MAX, other_move_capture.clone()),
        WalkType::new(vec![0, 1], usize::MAX, other_move_capture)
    ];
    let default_king_move = vec![
        WalkType::new(vec![1, 1], 1, other_move_capture_threatened.clone()),
        WalkType::new(vec![1, 0], 1, other_move_capture_threatened.clone()),
        WalkType::new(vec![1, -1], 1, other_move_capture_threatened.clone()),
        WalkType::new(vec![0, -1], 1, other_move_capture_threatened.clone()),
        WalkType::new(vec![-1, -1], 1, other_move_capture_threatened.clone()),
        WalkType::new(vec![-1, 0], 1, other_move_capture_threatened.clone()),
        WalkType::new(vec![-1, 1], 1, other_move_capture_threatened.clone()),
        WalkType::new(vec![0, 1], 1, other_move_capture_threatened),

    ];
    HashMap::from([
        (
            white_pawn, vec![
            WalkType::new(vec![0, 1], 1, HashMap::from([("move_type".to_string(), vec!["move".to_string()])])),
            WalkType::new(vec![1, 1], 1, HashMap::from([("move_type".to_string(), vec!["capture".to_string()])])),
            WalkType::new(vec![-1, 1], 1, HashMap::from([("move_type".to_string(), vec!["capture".to_string()])]))
        ]
        ),
        (
            black_pawn, vec![
            WalkType::new(vec![0, -1], 1, HashMap::from([("move_type".to_string(), vec!["move".to_string()])])),
            WalkType::new(vec![1, -1], 1, HashMap::from([("move_type".to_string(), vec!["capture".to_string()])])),
            WalkType::new(vec![-1, -1], 1,  HashMap::from([("move_type".to_string(), vec!["capture".to_string()])]))
        ]
        ),
        (white_knight, default_knight_move.clone()),
        (black_knight, default_knight_move),
        (white_bishop, default_bishop_move.clone()),
        (black_bishop, default_bishop_move),
        (white_rook, default_rook_move.clone()),
        (black_rook, default_rook_move),
        (white_queen, default_queen_move.clone()),
        (black_queen, default_queen_move),
        (white_king, default_king_move.clone()),
        (black_king, default_king_move)
    ])
}

pub fn default_setting() -> (Board2D, Vec<String>, HashMap<Piece, Vec<WalkType2D>>) {
    (default_board(), default_piece_type(), default_piece_move())
}

fn custom_calculate_moved<const D: usize>(board: BoardXD<D>, piece_type: Vec<String>, piece_direction: HashMap<Piece, Vec<WalkType<D>>>, deep: usize) -> CanMove<D> {
    MainCalculate::new(board, piece_type, piece_direction).calculate_moved(deep)
}

fn chess_x_convent(input: String) -> usize {
    input.parse().unwrap()
}

fn chess_y_convent(input: String) -> usize {
    input.chars().enumerate().map(|(radix, num)| (num.to_digit(36).unwrap() - 9) as usize * 26_usize.pow(radix as u32)).sum::<usize>() - 1
}

pub fn check_move_2d(moves: Vec<&MoveType2D>, player_input: String) -> Option<Vec<MoveType2D>> {
    todo!()
}

pub fn check_move<const D: usize>(moves: Vec<&MoveType<D>>, player_input: String) -> Option<Vec<MoveType<D>>> {
    /*
    if D == 2 {
        check_move_2d(moves, player_input)
    } else {
        None
    }
    */
    todo!()
}

fn custom_check_move<const D: usize>(board: BoardXD<D>, piece_type: Vec<String>, piece_move: HashMap<Piece, Vec<WalkType<D>>>, player_input: String) -> Option<Vec<MoveType<D>>> {
    check_move(custom_calculate_moved(board, piece_type, piece_move, 1).as_can_moves().unwrap().1.keys().collect(), player_input)
}
