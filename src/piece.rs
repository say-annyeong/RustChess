use std::{
    fmt::{self, Display, Formatter},
    collections::{
        HashMap,
        hash_map::Entry,
        BTreeMap,
        HashSet
    },
    sync::Arc,
    any::Any,
    vec
};
use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
use rayon::prelude::{ParallelIterator, IntoParallelIterator};
use regex::Regex;
use lazy_static::lazy_static;
use proc_lib::Dimension;

pub type Board2D = BoardXD<2>;
pub type MoveType2D = MoveData<2>;
pub type WalkType2D = WalkType<2>;
pub type CalculateMoves2D<'a> = CalculateMoves<'a, 2>;
pub type MainCalculate2D = MainCalculate<2>;
pub type ParsePlayerInput2D = ParsePlayerInput<2>;
pub type CanMove2D = CanMove<2>;
pub type OtherMovementType = String;

/*
Piece
color_name
short_color_name
piece_name
piece_short_name

WalkType
move_type: move, capture, threatened
attributes: check, checkmate
*/
/*
구조 싹다 바꿔!
*/

/*
이름: 폭군 (킹)
능력: 템페스트 룩+나이트+퀸
특별룰: 킹 대신 쓰며 킹이랑 똑같이 잡히면 바로 게임 끝.
퀸이 살아 있을 때만 이동 가능.

이름: 반란군 (폰)
능력: 폰이랑 동일
특별룰: 킹으로 프로모션 가능.

이름: 암행어사
특별룰: 기물 중 랜덤으로 암행어사가 됨. 그 기물을 잡을시 같이 잡힌다.

이름 : 졸
위, 좌, 우로 take-move 행마법

이름: 드론
행마법: 퀸이랑 동일
특수룰: 나이트 처럼 기물을 뛰어 넘을수 있음.
점수: 11점

이름:스나이퍼 행마법:비숍과 동일 특수룰: 게임당 한번 직선상에 있는 적 하나 잡을수 있음 점수는 한 4점?

이름 :아처
행마법 : 주위 3x3 이동만 가능 주위 5x5 공격만 가능
점수는 폰보단 높은 2점?

킹과 같은 파일에 있거나 같은 랭크에있고 사이에 막는 기물이 없으면  장거리 캐슬링 가능 단,킹의 이동 경로에 체크받는 기물있으면 이동 불가하고 둘다 한번이상 움직여도 상관없이 가능

이름:Telepotter
행마법:킹과동일
특수규칙:기본적으로 혼자이동이 불가능하지만주변8칸의 아군이있으면위치를바꾸고이동가능하고적이있으면 잡고이동가능

이름:Neutrator
행마법:아마존(마하라자)와동일
특수규칙:색깔은회색을띠며 백턴에는백이조종하고 흑턴에는흑이조종가능

이름:Gimcy
행마법:킹과동일
특수규칙:기물을Gimcy로잡을때마다코인획득 기물의따라 주는코인이다름
주는코인:폰은1원,나머진 기물점수-1코인,그리고 코인으로기물구입 가능 가격은기물점수만큼 소환위치는 기물이없는칸중하나선택으로선정

이름:turtle
행마법:킹과동일
특수규칙:수가홀수일때만움직이기 가능
예를들어첫수는홀수이기 때문에이동가능 하지만두번째수는짝수이기 때문에이동 불가능

이름:rabbit
행마법:나이트와동일
특수규칙:한턴에두번이동가능

이름:Sea turtle
특수규칙:전체8×8(64칸)중램덤으로40칸이바다로지정나머진육지로지정 육지위에선turtle행마법으로이동하지만 바다위에선rabbit행마법으로이동

이름:Night runaway
행마법:나이트와동일
특수규칙:기물을뛰어넘을때 중간에있는기물을아군이든적군이든잡음그리고 이동했을때기물을 잡았으면 한번더이동가능 또잡았으면 또이동가능

이름:criminal
행마법:폰과동일
특수규칙:적을잡을수없고프로모션이가능한데 프로모션시 적기물이됌

이름:fraud
행마법:퀸과동일
특수규칙:적에겐 킹으로보임

이름:Voice phishing
행마법:폰과동일
특수규칙:기물을잡을경우잡은기물로보임

이름:Gambler
특수규칙:시작시 폰,룩,비숍,나이트,퀸,킹의행마법중 랜덤으로하나로이동 한번움직일때마다 행마법이랜덤으로변경됌

창작체스기물:Dragon
행마법:퀸+나이트+카멜레온(미러링은적용하지않음)그리고다 뛰어넘을수있음

기물 이름: 회귀자(returner)
기물 행마법: 상하좌우 대각선으로 2칸 이내로 이동 + 나이트 행마법(take-move) (기물을 뛰어넘을 수 있음)
특수규칙: 해당 기물이 잡히면 5수 전의 위치로 이동. 단, 해당 칸에 다른 기물이 존재하면 해당 기물은 회귀가 불가능하다. 5수 내로 잡혀도 회귀가 불가능하다.
예상 점수: 잡히기 어렵다는 점을 생각 해 보았을 때 7점이 적절하다고 생각됨

스펙터(Specter)
모양: 반투명한 유령 형태, 머리 위에 작은 왕관처럼 빛나는 고리.
행마법:
대각선으로 한 칸 이동.
적 기물 위를 “통과”해 다음 칸으로 갈
수 있음(단, 착지하는 칸은 비어있어야 함).
특징: 스펙터가 통과한 적 기물은 다음 턴 동안 움직이지 못함.

포르티스(Fortis)
모양: 성벽처럼 네모난 탑, 중앙에 빛나는 보석.
행마법:
룩처럼 직선으로 이동하지만 최대 3칸까지만.
자신이 지나간 칸에 ‘방패 토큰’을 1턴 동안 남김.
특징: 방패 토큰이 있는 칸의 아군 기물은 1턴 동안 잡히지 않음.

위스퍼(Whisper)
모양: 깃털 달린 마법 모자, 아래쪽은 바람처럼 흩날리는 형상.
행마법:
나이트처럼 ‘ㄱ’자로 이동.
착지 시 그 주변 1칸(8방향) 안의 적 기물의 시야를 차단해, 그 기물은 다음 턴 동안 이동 범위가 1칸 줄어듦.

이름:로그체스(로그라이크+체스)
분류:특수룰
룰:총 다섯 판으로 진행하며 한판에서 질때마다 '특수 능력' 3가지를 뽑는다. 그중에 하나를 뽑아 적용한다. 특수능력은... 아무렇게나 하면 되겠죠? 왠만하면 조건부로 하면 좋을듯 싶군요.

이후 4판이 끝나면 마지막 결승을  시작하며 이때는 각각 플레이어의 특수 능력을 한 장식 교환한다

Push
이 행마를 가진 기물이 바라보는 방향대로, 막히지 않는 한 원하는 만큼 밀어냄

기보
F라는 기물이 e2에서 e3폰을 7랭크로 밀어냄->
Fpe3-7

예시 기물
선풍기(Fan)
상하좌우 1칸씩 move&push
대각선 4방향으로 한 칸씩 take
move와 push는 한 턴에 하나만 할 수 있음

⬜️⬛️⬜️⬆️⬜️⬛️⬜️⬛️
⬛️⬜️⬛️⬆️⬛️⬜️⬛️⬜️
⬜️⬛️⬜️⬆️⬜️⬛️⬜️⬛️
⬛️⬜️❌⭕❌⬜⬛⬜️
⬅️⬅️⭕️⚛️⭕️➡️➡️➡️
⬛️⬜❌⭕❌⬜⬛⬜️
⬜⬛⬜⬇️⬜⬛⬜⬛️
⬛⬜⬛⬇️⬛⬜⬛⬜️
⭕️=move&push
❌️=take
⬆️⬇️⬅️➡️=push로 밀어낼 수 있는 방향
⚛️=선풍기 이모티콘이 없음

Ride
다른 기물에 업힘(탑승함)

기보
H라는 기물이 f3퀸에 탑승함->
Hrf3

예시 기물
매(Hawk)
대각선 4방향 제외, 자신과 2개 떨어진 칸을 catch
자신 주변 8칸을 ride로 이동
move 불가
⬜️ ⬛️ ⬜️ ⬛️ ⬜️ ⬛️ ⬜️ ⬛️
⬛️ ⬜️ ⬛️ ⬜️ ⬛️ ⬜️ ⬛️ ⬜️
⬜️ ⬛️ 🔺️ 🔺️ 🔺️ ⬛️ ⬜️ ⬛️
⬛️ 🔺️⬇️⬇️⬇️🔺️ ⬛️ ⬜️
⬜️ 🔺️⬇️🦅⬇️🔺️ ⬜️ ⬛️
⬛️ 🔺️⬇️⬇️⬇️🔺️ ⬛️ ⬜️
⬜️ ⬛️ 🔺️ 🔺️ 🔺️ ⬛️ ⬜️ ⬛️
⬛️ ⬜️ ⬛️ ⬜️ ⬛️ ⬜️ ⬛️ ⬜️
⬇️=ride
🔺️=catch

Bard

행마법: 이미지

아군을 공격하여 바드의 공격을 받은 아군이 한번에 한하여 한번 더 움직일수 있게 해줍니다.

적은 잡지 못합니다.

Thrust
이 행마를 가진 기물은 밀어내는 방향이 막혀있지 않다면 다른 아군 기물로 밀어낼 수 있음

기보
다른 기물이 있어야 가능한 행마라 따로 표기하지 않음

예시 기물
돌덩이(Rock)
시시포스가 계속 굴리는 그 돌
세로로 막히지 않는 한 원하는 만큼 move&take
단, 폰은 관통 가능
⬜️ ⬛️ ⬜️ ⭕️ ⬜️ ⬛️ ⬜️ ⬛️
⬛️ ⬜️ ⬛️ ⭕️ ⬛️ ⬜️ ⬛️ ⬜️
⬜️ ⬛️ ⬜️ ⭕️ ⬜️ ⬛️ ⬜️ ⬛️
⬛️ ⬜️ ⬛️ ⭕️ ⬛️ ⬜️ ⬛️ ⬜️
⬜️ ⬛️ ⬜️🪨 ⬜️ ⬛️ ⬜️ ⬛️
⬛️ ⬜️ ⬛️ ⭕️ ⬛️ ⬜️ ⬛️ ⬜️
⬜️ ⬛️ ⬜️ ⭕️ ⬜️ ⬛️ ⬜️ ⬛️
⬛️ ⬜️ ⬛️ ⭕️ ⬛️ ⬜️ ⬛️ ⬜️
⭕️=move&take

아래랑 메시지 나눠놨어요
ㅡㅡㅡ
Anchor
자신을 잡은 기물을 다음 턴까지(턴 수는 기물에 따라 바뀔 수 있음) 묶어둠(고정시킴)
퀸 등 좋은 기물의 길을 막는 데 쓰임
킹이 이 행마를 가진 기물을 잡으면 킹으로 킹을 잡을 수 있음

기보
따로 표기하지 않음

예시 기물
슬라임(Slime)
8방향으로 move(take x)
1턴 만큼 anchor
⬜️ ⬛️ ⬜️ ⬛️ ⬜️ ⬛️ ⬜️ ⬛️
⬛️ ⬜️ ⬛️ ⬜️ ⬛️ ⬜️ ⬛️ ⬜️
⬜️ ⬛️ ⬜️ ⬛️ ⬜️ ⬛️ ⬜️ ⬛️
⬛️ ⬜️ ⭕️ ⭕️ ⭕️ ⬜️ ⬛️ ⬜️
⬜️ ⬛️ ⭕️ 🦠 ⭕️ ⬛️ ⬜️ ⬛️
⬛️ ⬜️ ⭕️ ⭕️ ⭕️ ⬜️ ⬛️ ⬜️
⬜️ ⬛️ ⬜️ ⬛️ ⬜️ ⬛️ ⬜️ ⬛️
⬛️ ⬜️ ⬛️ ⬜️ ⬛️ ⬜️ ⬛️ ⬜️
⭕️=move

Stun
턴을 소모해 일정 턴 동안 기물을 기절시킴(고정시킴). Hold의 하위호환.
Hold에 비해 범위를 좀 더 넓게 잡을 수 있음

기보
L이라는 기물이 주위 8칸을 기절시킴->
Ls

예시 기물
번개(Lightning)
아군 포함, 주위 8칸을 한 번에 stun
대각선으로 막히지 않는 한 원하는 만큼 move(take x)
한 턴에 move와 stun 둘 중 하나만 가능
⬜️ ⬛️ ⬜️ ⬛️ ⬜️ ⬛️ ⬜️ ⭕️
⭕️ ⬜️ ⬛️ ⬜️ ⬛️ ⬜️ ⭕️ ⬜️
⬜️ ⭕️ ⬜️ ⬛️ ⬜️ ⭕️ ⬜️ ⬛️
⬛️ ⬜️💫💫💫⬜️ ⬛️ ⬜️
⬜️ ⬛️💫 ⚡️💫⬛️ ⬜️ ⬛️
⬛️ ⬜️💫💫💫⬜️ ⬛️ ⬜️
⬜️ ⭕️ ⬜️ ⬛️ ⬜️ ⭕️ ⬜️ ⬛️
⭕️ ⬜️ ⬛️ ⬜️ ⬛️ ⬜️ ⭕️ ⬜️
⭕️=move
💫=stun
ㅡㅡㅡ
그냥 기물
신호등
가로로 막히지 않는 한 원하는 만큼 move&take
한 턴 마다 색이 바뀜
초록색일 땐 세로로 한 칸 씩 move
노란색일 땐 앞 3칸 barrier
빨간색일 땐 앞 3칸 barrier&hold
첫 턴에는 초록색
⬜️ ⬛️ ⬜️ ⬛️ ⬜️ ⬛️ ⬜️ ⬛️
⬛️ ⬜️ ⬛️⏸️⬛️ ⬜️ ⬛️ ⬜️
⬜️ ⬛️ ⬜️⏸️⬜️ ⬛️ ⬜️ ⬛️
⬛️ ⬜️ ⬛️⏸️⬛️ ⬜️ ⬛️ ⬜️
❌️ ❌️❌️🚦❌️ ❌️ ❌️ ❌️
⬛️ ⬜️ ⬛️ ⭕️ ⬛️ ⬜️ ⬛️ ⬜️
⬜️ ⬛️ ⬜️ ⬛️ ⬜️ ⬛️ ⬜️ ⬛️
⬛️ ⬜️ ⬛️ ⬜️ ⬛️ ⬜️ ⬛️ ⬜️
❌️=move&take
⭕️=move
⏸️=barrier&hold


*/

lazy_static! {
    static ref PLAYER_INPUT_RE: Regex = Regex::new(
        r"(?P<name>[A-Za-z]*)(?P<start_col>[A-Za-z]*)(?P<start_row>\d*)(?P<takes>[Xx]?)(?P<end_col>[A-Za-z]+)(?P<end_row>\d+)(?P<other>.*)"
    ).unwrap();
    static ref OTHER_MOVE_CAPTURE: HashMap<String, Vec<String>> = HashMap::from([("move_type".to_string(), vec!["move".to_string(), "capture".to_string()])]);
    static ref OTHER_MOVE_CAPTURE_THREATENED: HashMap<String, Vec<String>> = HashMap::from([("move_type".to_string(), vec!["move".to_string(), "capture".to_string(), "threatened".to_string()])]);
}

macro_rules! default_pieces {
    ($white_pawn:ident, $white_knight:ident, $white_bishop:ident, $white_rook:ident, $white_queen:ident, $white_king:ident, $black_pawn:ident, $black_knight:ident, $black_bishop:ident, $black_rook:ident, $black_queen:ident, $black_king:ident) => {
        let $white_pawn = Piece::pawn("white".to_string(), vec!["W".to_string()]);
        let $white_knight = Piece::knight("white".to_string(), vec!["W".to_string()]);
        let $white_bishop = Piece::bishop("white".to_string(), vec!["W".to_string()]);
        let $white_rook = Piece::rook("white".to_string(), vec!["W".to_string()]);
        let $white_queen = Piece::queen("white".to_string(), vec!["W".to_string()]);
        let $white_king = Piece::king("white".to_string(), vec!["W".to_string()]);

        let $black_pawn = Piece::pawn("black".to_string(), vec!["B".to_string()]);
        let $black_knight = Piece::knight("black".to_string(), vec!["B".to_string()]);
        let $black_bishop = Piece::bishop("black".to_string(), vec!["B".to_string()]);
        let $black_rook = Piece::rook("black".to_string(), vec!["B".to_string()]);
        let $black_queen = Piece::queen("black".to_string(), vec!["B".to_string()]);
        let $black_king = Piece::king("black".to_string(), vec!["B".to_string()]);
    };
}

trait Dimension<const D: usize> {
    fn dimensions() -> usize;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Dimension)]
struct AbsolutePosition<const D: usize> {
    position: [usize; D],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Dimension)]
struct RelativePosition<const D: usize> {
    offset: [isize; D],
}

impl<const D: usize> AbsolutePosition<D> {
    fn to_relative(&self, target: &AbsolutePosition<D>) -> RelativePosition<D> {
        let mut offset = [0; D];
        for i in 0..D {
            offset[i] = target.position[i] as isize - self.position[i] as isize;
        }
        RelativePosition { offset }
    }

    fn add_absolute(&self, rel: &RelativePosition<D>) -> Option<AbsolutePosition<D>> {
        let mut new_pos = [0; D];
        for i in 0..D {
            let new_val = self.position[i] as isize + rel.offset[i];
            if new_val < 0 {
                return None; // 범위 초과
            }
            new_pos[i] = new_val as usize;
        }
        Some(AbsolutePosition { position: new_pos })
    }
}

impl<const D: usize> From<[usize; D]> for AbsolutePosition<D> {
    fn from(value: [usize; D]) -> Self {
        Self { position: value }
    }
}

impl<const D: usize> From<AbsolutePosition<D>> for [usize; D] {
    fn from(value: AbsolutePosition<D>) -> Self {
        value.position
    }
}

impl<const D: usize> TryFrom<Vec<usize>> for AbsolutePosition<D> {
    type Error = Vec<usize>;

    fn try_from(value: Vec<usize>) -> Result<Self, Vec<usize>> {
        match value.try_into() {
            Ok(position) => Ok(Self { position }),
            Err(e) => Err(e),
        }
    }
}

impl<const D: usize> Deref for AbsolutePosition<D> {
    type Target = [usize; D];

    fn deref(&self) -> &[usize; D] {
        &self.position
    }
}

impl<const D: usize> DerefMut for AbsolutePosition<D> {
    fn deref_mut(&mut self) -> &mut [usize; D] {
        &mut self.position
    }
}

impl<const D: usize> AsRef<[usize; D]> for AbsolutePosition<D> {
    fn as_ref(&self) -> &[usize; D] {
        &self.position
    }
}

impl<const D: usize> AsMut<[usize; D]> for AbsolutePosition<D> {
    fn as_mut(&mut self) -> &mut [usize; D] {
        &mut self.position
    }
}

impl<const D: usize> RelativePosition<D> {
    fn to_absolute(&self, base: &AbsolutePosition<D>) -> Option<AbsolutePosition<D>> {
        base.add_absolute(self)
    }

    fn from_absolute(base: &AbsolutePosition<D>, target: &AbsolutePosition<D>) -> Self {
        base.to_relative(target)
    }
}

impl<const D: usize> From<[isize; D]> for RelativePosition<D> {
    fn from(value: [isize; D]) -> Self {
        Self { offset: value }
    }
}

impl<const D: usize> From<RelativePosition<D>> for [isize; D] {
    fn from(value: RelativePosition<D>) -> Self {
        value.offset
    }
}

impl<const D: usize> TryFrom<Vec<isize>> for RelativePosition<D> {
    type Error = Vec<isize>;

    fn try_from(value: Vec<isize>) -> Result<Self, Vec<isize>> {
        match value.try_into() {
            Ok(offset) => Ok(Self { offset }),
            Err(e) => Err(e),
        }
    }
}

impl<const D: usize> Deref for RelativePosition<D> {
    type Target = [isize; D];

    fn deref(&self) -> &[isize; D] {
        &self.offset
    }
}

impl<const D: usize> DerefMut for RelativePosition<D> {
    fn deref_mut(&mut self) -> &mut [isize; D] {
        &mut self.offset
    }
}

impl<const D: usize> AsRef<[isize; D]> for RelativePosition<D> {
    fn as_ref(&self) -> &[isize; D] {
        &self.offset
    }
}

impl<const D: usize> AsMut<[isize; D]> for RelativePosition<D> {
    fn as_mut(&mut self) -> &mut [isize; D] {
        &mut self.offset
    }
}

#[derive(Dimension)]
struct MovingEventCondition<const D: usize> {
    condition: String
}

#[derive(Dimension)]
struct MovingEventAction<const D: usize> {
    action: String
}

#[derive(Dimension)]
struct MovingEvent<const D: usize> {
    condition: MovingEventCondition<D>,
    action: MovingEventAction<D>
}

#[derive(Dimension)]
struct MovingRule<const D: usize> {
    center_positions: RelativePosition<D>,
    delta_positions: RelativePosition<D>,
    repeat: usize,
    moving_event: MovingEvent<D>
}

/*
impl Iterator for MovingRule<2> {
    type Item = AbsolutePosition<2>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_move()
    }
}
*/

#[derive(Dimension)]
struct MoveType<const D: usize> {
    c_positions: AbsolutePosition<D>,
    moving_rule: MovingRule<D>,
    default_movement_type: HashSet<DefaultMovementType>,
    custom_movement_type: HashSet<CustomMovementType>,
    other_movement_type: HashSet<OtherMovementType>,
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

    fn pawn(color: String, short_color: Vec<String>) -> Self {
        Self::new(color, "pawn".to_string(), BTreeMap::from([("attributes".to_string(), vec!["promotion".to_string()]), ("short_name".to_string(), vec!["P".to_string()]), ("short_color_name".to_string(), short_color)]))
    }

    fn knight(color: String, short_color: Vec<String>) -> Self {
        Self::new(color, "knight".to_string(), BTreeMap::from([("short_name".to_string(), vec!["N".to_string()]), ("short_color_name".to_string(), short_color)]))
    }

    fn bishop(color: String, short_color: Vec<String>) -> Self {
        Self::new(color, "bishop".to_string(), BTreeMap::from([("short_name".to_string(), vec!["B".to_string()]), ("short_color_name".to_string(), short_color)]))
    }

    fn rook(color: String, short_color: Vec<String>) -> Self {
        Self::new(color, "rook".to_string(), BTreeMap::from([("short_name".to_string(), vec!["R".to_string()]), ("short_color_name".to_string(), short_color)]))
    }

    fn queen(color: String, short_color: Vec<String>) -> Self {
        Self::new(color, "queen".to_string(), BTreeMap::from([("short_name".to_string(), vec!["Q".to_string()]), ("short_color_name".to_string(), short_color)]))
    }

    fn king(color: String, short_color: Vec<String>) -> Self {
        Self::new(color, "king".to_string(), BTreeMap::from([("attributes".to_string(), vec!["check".to_string(), "checkmate".to_string()]), ("short_name".to_string(), vec!["K".to_string()]), ("short_color_name".to_string(), short_color)]))
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
    board_size: [usize; D],
    pieces: HashMap<AbsolutePosition<D>, (Piece, HashMap<String, Vec<String>>)>
}

impl<const D: usize> BoardXD<D> {
    pub fn new(board_size: [usize; D], pieces: HashMap<AbsolutePosition<D>, (Piece, HashMap<String, Vec<String>>)>) -> Self {
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
                let Some((piece, _other)) = self.pieces.get(&AbsolutePosition::from([y, x])) else {
                    write!(f, " -")?;
                    continue
                };
                write!(f, "{}", piece)?;
            }
            writeln!(f)?;
        }
        Ok(())
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
pub struct MoveData<const D: usize> {
    center_positions: Option<AbsolutePosition<D>>,
    positions: Option<AbsolutePosition<D>>,
    move_type: Option<String>,
    piece: Option<Piece>,
    takes_piece: Option<Piece>,
    other: Option<BTreeMap<String, Vec<String>>>
}

impl<const D: usize> MoveData<D> {
    pub fn new(center_positions: Option<AbsolutePosition<D>>, positions: Option<AbsolutePosition<D>>, move_type: Option<String>,
               piece: Option<Piece>, takes_piece: Option<Piece>, other: Option<BTreeMap<String, Vec<String>>>) -> Self {
        Self { center_positions, positions, move_type, piece, takes_piece, other }
    }

    fn all_none_as_except_other(&self) -> bool {
        self.center_positions == None && self.positions == None && self.move_type == None && self.piece == None && self.takes_piece == None
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
    delta_positions: RelativePosition<D>,
    times: usize,
    other: HashMap<String, Vec<String>>
}

impl<const D: usize> WalkType<D> {
    fn new(delta_positions: RelativePosition<D>, times: usize, other: HashMap<String, Vec<String>>) -> Self {
        Self { delta_positions, times, other }
    }
}

impl WalkType2D {
    fn knight() -> Vec<Self> {
        vec![
            Self::new([2, 1].into(), 1, OTHER_MOVE_CAPTURE.clone()),
            Self::new([2, -1].into(), 1, OTHER_MOVE_CAPTURE.clone()),
            Self::new([1, -2].into(), 1, OTHER_MOVE_CAPTURE.clone()),
            Self::new([-1, -2].into(), 1, OTHER_MOVE_CAPTURE.clone()),
            Self::new([-2, -1].into(), 1, OTHER_MOVE_CAPTURE.clone()),
            Self::new([-2, 1].into(), 1, OTHER_MOVE_CAPTURE.clone()),
            Self::new([-1, 2].into(), 1, OTHER_MOVE_CAPTURE.clone()),
            Self::new([1, 2].into(), 1, OTHER_MOVE_CAPTURE.clone())
        ]
    }

    fn bishop() -> Vec<Self> {
        vec![
            Self::new([1, 1].into(), usize::MAX, OTHER_MOVE_CAPTURE.clone()),
            Self::new([1, -1].into(), usize::MAX, OTHER_MOVE_CAPTURE.clone()),
            Self::new([-1, -1].into(), usize::MAX, OTHER_MOVE_CAPTURE.clone()),
            Self::new([-1, 1].into(), usize::MAX, OTHER_MOVE_CAPTURE.clone())
        ]
    }

    fn rook() -> Vec<Self> {
        vec![
            Self::new([1, 0].into(), usize::MAX, OTHER_MOVE_CAPTURE.clone()),
            Self::new([0, -1].into(), usize::MAX, OTHER_MOVE_CAPTURE.clone()),
            Self::new([-1, 0].into(), usize::MAX, OTHER_MOVE_CAPTURE.clone()),
            Self::new([0, 1].into(), usize::MAX, OTHER_MOVE_CAPTURE.clone())
        ]
    }

    fn queen() -> Vec<Self> {
        vec![
            Self::new([1, 1].into(), usize::MAX, OTHER_MOVE_CAPTURE.clone()),
            Self::new([1, 0].into(), usize::MAX, OTHER_MOVE_CAPTURE.clone()),
            Self::new([1, -1].into(), usize::MAX, OTHER_MOVE_CAPTURE.clone()),
            Self::new([0, -1].into(), usize::MAX, OTHER_MOVE_CAPTURE.clone()),
            Self::new([-1, -1].into(), usize::MAX, OTHER_MOVE_CAPTURE.clone()),
            Self::new([-1, 0].into(), usize::MAX, OTHER_MOVE_CAPTURE.clone()),
            Self::new([-1, 1].into(), usize::MAX, OTHER_MOVE_CAPTURE.clone()),
            Self::new([0, 1].into(), usize::MAX, OTHER_MOVE_CAPTURE.clone())
        ]
    }

    fn king() -> Vec<Self> {
        vec![
            Self::new([1, 1].into(), 1, OTHER_MOVE_CAPTURE_THREATENED.clone()),
            Self::new([1, 0].into(), 1, OTHER_MOVE_CAPTURE_THREATENED.clone()),
            Self::new([1, -1].into(), 1, OTHER_MOVE_CAPTURE_THREATENED.clone()),
            Self::new([0, -1].into(), 1, OTHER_MOVE_CAPTURE_THREATENED.clone()),
            Self::new([-1, -1].into(), 1, OTHER_MOVE_CAPTURE_THREATENED.clone()),
            Self::new([-1, 0].into(), 1, OTHER_MOVE_CAPTURE_THREATENED.clone()),
            Self::new([-1, 1].into(), 1, OTHER_MOVE_CAPTURE_THREATENED.clone()),
            Self::new([0, 1].into(), 1, OTHER_MOVE_CAPTURE_THREATENED.clone())
        ]
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
    piece_direction: &'a HashMap<Piece, Vec<WalkType<D>>>
}

impl<'a, const D: usize> CalculateMoves<'a, D> {
    fn new(board: BoardXD<D>, piece_direction: &'a HashMap<Piece, Vec<WalkType<D>>>) -> Self {
        Self { board, piece_direction }
    }

    fn step(&self, positions: AbsolutePosition<D>, walk_type: WalkType<D>) -> MoveData<D> {
        match walk_type.other.get(&"move_type".to_string()) {
            Some(move_type) => {
                if let Some((piece, _other)) = self.board.pieces.get(&positions) {
                    if move_type.contains(&"capture".to_string()) {
                        return MoveData::new(
                            None,
                            Some(positions),
                            Some("x".into()),
                            None,
                            Some(piece.clone()),
                            Some(walk_type.other.into_iter().collect())
                        )
                    }
                } else {
                    // 해당 위치에 기물이 없을 경우 "move" 이동을 확인
                    if move_type.contains(&"move".to_string()) {
                        return MoveData::new(
                            None,
                            Some(positions),
                            Some("m".into()),
                            None,
                            None,
                            Some(walk_type.other.into_iter().collect())
                        )
                    }
                }
            }
            None => return MoveData::default()
        }

        MoveData::default()
    }

    fn walk(&self, c_positions: AbsolutePosition<{ D }>, piece_walk_types: (Piece, Vec<WalkType<D>>)) -> Vec<MoveData<D>> {
        let (piece, walk_types) = piece_walk_types;

        walk_types.into_par_iter().flat_map(|walk_type| {
            let mut moves = Vec::new();
            let mut positions = c_positions.clone();
            let mut jump = 0;

            'walk_loop: for _ in 0..walk_type.times {
                let next_position: Option<Vec<_>> = positions.iter()
                    .zip(walk_type.delta_positions.iter())
                    .map(|(x, dx)| *x as isize + dx)
                    .map(|x| if x < 0 { None } else { Some(x as usize) })
                    .collect();
                let Some(next_positions) = next_position else { break };

                if next_positions.iter().zip(&self.board.board_size).any(|(x, mx)| x >= mx) { break }

                if c_positions.iter().zip(&next_positions).all(|(cx, x)| cx == x) { continue }

                let mut moving = self.step(AbsolutePosition::try_from(next_positions.clone()).unwrap(), walk_type.clone());

                match moving.all_none_as_except_other() {
                    true => {
                        if let Some(other) = moving.other {
                            let Some(attribute) = other.get(&"attribute".to_string()) else {
                                break 'walk_loop
                            };
                            if attribute.contains(&"jump_1".to_string()) && jump == 0 {
                                jump += 1;
                                continue 'walk_loop
                            } else {
                                break 'walk_loop
                            }
                        } else {
                            break 'walk_loop
                        }
                    },
                    false => {
                        moving.piece = Some(piece.clone());
                        moving.center_positions = Some(c_positions.clone());
                        moves.push(moving.clone());
                    }
                }
                positions = next_positions.try_into().unwrap();
            }
            moves
        }).collect()
    }

    fn piece(self: Arc<Self>, positions: AbsolutePosition<D>) -> Vec<MoveData<D>> {
        let Some((piece, _)) = &self.board.pieces.get(&positions) else {
            return Vec::new();
        };

        let (board_color, board_piece_type) = (&piece.color, &piece.name);

        // 병렬 반복자로 변환하여 필터링 및 매핑 수행
        self.piece_direction.clone()
            .into_par_iter()
            .filter_map(|walk_type| {
                let (piece, _other) = &walk_type;
                let (walk_type_color, walk_type_piece_type) = (&piece.color, &piece.name);

                if board_color == walk_type_color && board_piece_type == walk_type_piece_type {
                    Some(self.walk(positions.clone(), walk_type))
                } else {
                    None
                }
            })
            .flatten()
            .collect()
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
                let cache = Arc::new(Self::new(board, self.piece_direction));
                (cache.search_piece(deep - 1), moving)
            }).collect();

            for (can_move, moving) in buffer {
                output.insert(moving, Box::new(can_move));
            }
        } else {
            for moving in piece_search {
                let moved_board = self.piece_moved(moving.clone());
                output.insert(moving, Box::new(CanMove::Board(moved_board)));
            }
        }
        CanMove::CanMoves((self.board.clone(), output))
    }

    fn piece_moved(&self, move_type: MoveData<D>) -> BoardXD<D> {
        let mut buffer = self.board.clone();

        if let (Some(c_positions), Some(positions)) = (move_type.center_positions, move_type.positions) {

            buffer.pieces.iter_mut().for_each(|(_k, (_t, statuses))| {
                for (_k, v) in statuses.iter_mut() {
                    v.retain(|x| x != "moving")
                }
            });

            if let Some(piece) = buffer.pieces.get(&c_positions) {
                let mut piece = piece.clone();
                piece.1.entry("attributes".to_string()).and_modify(|v| v.push("moving".to_string())).or_insert(vec!["moving".to_string()]);

                match buffer.pieces.entry(positions) {
                    Entry::Occupied(_) => return buffer,
                    Entry::Vacant(entry) => entry.insert(piece)
                };

                buffer.pieces.remove_entry(&c_positions);
                buffer
            } else {
                buffer
            }
        } else {
            buffer
        }
    }
}

#[derive(Dimension)]
pub struct MainCalculate<const D: usize> {
    pub(crate) board: BoardXD<D>,
    piece_type: Vec<String>,
    piece_direction: HashMap<Piece, Vec<WalkType<D>>>,
    pub save_moves: CanMove<D>
}

impl<const D: usize> MainCalculate<D> {
    pub fn new(board: BoardXD<D>, piece_type: Vec<String>, piece_direction: HashMap<Piece, Vec<WalkType<D>>>) -> Self {
        let save_moves = CanMove::None;
        Self { board, piece_type, piece_direction, save_moves }
    }

    pub fn piece_move(&mut self, move_type: MoveData<D>) {
        if let (Some(c_positions), Some(positions)) = (move_type.center_positions, move_type.positions) {
            let buffer = &mut self.board.pieces;
            if buffer.contains_key(&c_positions) {
                let Some(v_buffer) = buffer.get(&c_positions).cloned() else {
                    return
                };
                buffer.remove(&c_positions);
                buffer.insert(positions, v_buffer.clone().clone());
            }
        }
    }

    pub fn piece_moved(&self, move_type: MoveData<D>) -> BoardXD<D> {
        CalculateMoves::new(self.board.clone(), &self.piece_direction).piece_moved(move_type)
    }

    pub fn calculate_move(&mut self, deep: usize) {
        let calculate = Arc::new(CalculateMoves::new(self.board.clone(), &self.piece_direction));
        self.save_moves = calculate.search_piece(deep);
    }

    pub fn calculate_moved(&self, deep: usize) -> CanMove<D> {
        let calculate = Arc::new(CalculateMoves::new(self.board.clone(), &self.piece_direction));
        calculate.search_piece(deep)
    }

    pub fn continue_calculate_moves(&mut self, insert_can_move: &mut CanMove<D>) {
        todo!("할꺼야")
    }
}

impl Default for MainCalculate2D {
    fn default() -> Self {
        Self::new(default_board(), default_piece_type(), default_piece_move())
    }
}

#[derive(Dimension)]
pub struct ParsePlayerInput<const D: usize> {
    moves: Vec<MoveData<D>>
}

impl<const D: usize> ParsePlayerInput<D> {
    pub fn new(moves: Vec<MoveData<D>>) -> Self {
        Self { moves }
    }
}

impl ParsePlayerInput2D {
    pub fn parse_player_input(&self, player_input: String) -> Vec<MoveType2D> {
        if let Some(input) = PLAYER_INPUT_RE.captures(player_input.as_str()) {
            let (mut name, start_col, start_row, _takes, end_col, end_row, _other) = (input["name"].to_lowercase(), input["start_col"].to_lowercase(), input["start_row"].to_string(), !input["takes"].is_empty(), input["end_col"].to_lowercase(), input["end_row"].to_string(), input["other"].to_lowercase());
            let cx = if start_col.is_empty() { None } else { Some(chess_y_convent(start_col)) };
            let cy = if start_row.is_empty() { None } else { Some(chess_x_convent(start_row)) };
            let x = Some(chess_x_convent(end_row));
            let y = Some(chess_y_convent(end_col));

            let (player_c_positions, player_positions) = (vec![cy, cx], vec![y, x]);

            if name.is_empty() {
                name = "pawn".to_string();
            }

            let mut can_moves = Vec::new();

            macro_rules! correct_check {
                ($input1:expr, $input2:expr, $output:ident) => {
                    let $output = match $input2 {
                        Some(contains) => $input1.iter().zip(contains.deref()).all(|(p_pos, pos)|{
                            match p_pos {
                                Some(p) => p == pos,
                                None => true
                            }
                        }),
                        None => false
                    };
                };
            }

            for move_type in &self.moves {
                let name_correct = move_type.piece.iter().cloned().any(|move_type| move_type.name == name);
                let (c_positions, positions) = (&move_type.center_positions, &move_type.positions);

                correct_check!(player_c_positions, c_positions.as_ref(), c_positions_correct);
                correct_check!(player_positions, positions.as_ref(), positions_correct);

                //let takes_correct = if takes { Some("x".to_string()) } else { None } == move_type.move_type;

                if name_correct && c_positions_correct && positions_correct {
                    can_moves.push(move_type);
                }
            }

            can_moves.into_iter().cloned().collect()
        } else {
            vec![MoveData::other(Some(BTreeMap::from([("player_input".to_string(), vec![player_input])])))]
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
    CanMoves((BoardXD<D>, HashMap<MoveData<D>, Box<Self>>)),
    Board(BoardXD<D>),
    #[default] None
}

impl<const D: usize> CanMove<D> {
    pub fn as_can_moves(&self) -> Option<&(BoardXD<D>, HashMap<MoveData<D>, Box<CanMove<D>>>)> {
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

#[derive(Default)]
enum DefaultMovementType {
    #[default]
    None,
    Move,
    Take,
}

#[derive(Default)]
enum CustomMovementType {
    #[default]
    None,
    Catch,
    Jump,
    Void,
    Hold,
    Barrier,
    Transfer,
    Overlap,
    Shift
}

pub fn default_board() -> Board2D {
    default_pieces!(white_pawn, white_knight, white_bishop, white_rook, white_queen, white_king, black_pawn, black_knight, black_bishop, black_rook, black_queen, black_king);
    
    Board2D::new(
        [8, 8].into(),
        HashMap::from(
            [
                ([0, 0].into(), (white_rook.clone(), HashMap::new())),
                ([0, 1].into(), (white_knight.clone(), HashMap::new())),
                ([0, 2].into(), (white_bishop.clone(), HashMap::new())),
                ([0, 3].into(), (white_queen, HashMap::new())),
                ([0, 4].into(), (white_king, HashMap::new())),
                ([0, 5].into(), (white_bishop, HashMap::new())),
                ([0, 6].into(), (white_knight, HashMap::new())),
                ([0, 7].into(), (white_rook, HashMap::new())),
                ([1, 0].into(), (white_pawn.clone(), HashMap::new())), ([1, 1].into(), (white_pawn.clone(), HashMap::new())), ([1, 2].into(), (white_pawn.clone(), HashMap::new())), ([1, 3].into(), (white_pawn.clone(), HashMap::new())), ([1, 4].into(), (white_pawn.clone(), HashMap::new())), ([1, 5].into(), (white_pawn.clone(), HashMap::new())), ([1, 6].into(), (white_pawn.clone(), HashMap::new())), ([1, 7].into(), (white_pawn, HashMap::new())),
                ([6, 0].into(), (black_pawn.clone(), HashMap::new())), ([6, 1].into(), (black_pawn.clone(), HashMap::new())), ([6, 2].into(), (black_pawn.clone(), HashMap::new())), ([6, 3].into(), (black_pawn.clone(), HashMap::new())), ([6, 4].into(), (black_pawn.clone(), HashMap::new())), ([6, 5].into(), (black_pawn.clone(), HashMap::new())), ([6, 6].into(), (black_pawn.clone(), HashMap::new())), ([6, 7].into(), (black_pawn, HashMap::new())),
                ([7, 0].into(), (black_rook.clone(), HashMap::new())),
                ([7, 1].into(), (black_knight.clone(), HashMap::new())),
                ([7, 2].into(), (black_bishop.clone(), HashMap::new())),
                ([7, 3].into(), (black_queen, HashMap::new())),
                ([7, 4].into(), (black_king, HashMap::new())),
                ([7, 5].into(), (black_bishop, HashMap::new())),
                ([7, 6].into(), (black_knight, HashMap::new())),
                ([7, 7].into(), (black_rook, HashMap::new())),
            ]
        )
    )
}

pub fn default_piece_type() -> Vec<String> {
    vec!["pawn".to_string(), "knight".to_string(), "bishop".to_string(), "rook".to_string(), "queen".to_string(), "king".to_string()]
}

pub fn default_piece_move() -> HashMap<Piece, Vec<WalkType2D>> {
    default_pieces!(white_pawn, white_knight, white_bishop, white_rook, white_queen, white_king, black_pawn, black_knight, black_bishop, black_rook, black_queen, black_king);
    HashMap::from([
        (
            white_pawn, vec![
            WalkType::new([0, 1].into(), 1, HashMap::from([("move_type".to_string(), vec!["move".to_string()])])),
            WalkType::new([1, 1].into(), 1, HashMap::from([("move_type".to_string(), vec!["capture".to_string()])])),
            WalkType::new([-1, 1].into(), 1, HashMap::from([("move_type".to_string(), vec!["capture".to_string()])]))
        ]
        ),
        (
            black_pawn, vec![
            WalkType::new([0, -1].into(), 1, HashMap::from([("move_type".to_string(), vec!["move".to_string()])])),
            WalkType::new([1, -1].into(), 1, HashMap::from([("move_type".to_string(), vec!["capture".to_string()])])),
            WalkType::new([-1, -1].into(), 1,  HashMap::from([("move_type".to_string(), vec!["capture".to_string()])]))
        ]
        ),
        (white_knight, WalkType::knight()),
        (black_knight, WalkType::knight()),
        (white_bishop, WalkType::bishop()),
        (black_bishop, WalkType::bishop()),
        (white_rook, WalkType::rook()),
        (black_rook, WalkType::rook()),
        (white_queen, WalkType::queen()),
        (black_queen, WalkType::queen()),
        (white_king, WalkType::king()),
        (black_king, WalkType::king())
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
    (input.chars().enumerate().map(|(radix, c)| (c as u8 - 'a' as u8 + 1) * 26u8.pow(radix as u32)).sum::<u8>() - 1) as usize
}

pub fn check_move_2d(moves: Vec<&MoveType2D>, player_input: String) -> Option<Vec<MoveType2D>> {
    todo!()
}

pub fn check_move<const D: usize>(moves: Vec<&MoveData<D>>, player_input: String) -> Vec<MoveData<D>> {
    let parse_move = ParsePlayerInput::new(moves.into_iter().cloned().collect());
    //parse_move.parse_player_input(player_input)
    todo!()
}

fn custom_check_move<const D: usize>(board: BoardXD<D>, piece_type: Vec<String>, piece_move: HashMap<Piece, Vec<WalkType<D>>>, player_input: String) -> Vec<MoveData<D>> {
    check_move(custom_calculate_moved(board, piece_type, piece_move, 1).as_can_moves().unwrap().1.keys().collect(), player_input)
}
