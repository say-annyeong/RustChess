use crate::{
    traits::Dimension,
    piece::Piece,
    impl_add_sub_mul,
    impl_ops_add_sub_mul_assign,
    impl_from_try_from,
    impl_ops_refs,
    impl_try_from_iterator
};

/// ## AbsolutePosition
/// AbsolutePositon은 Board의 절대 좌표 a1을 기준으로 합니다.
/// 좌표 저장은 (D1, D2, D3...)의 순서로 저장됩니다.
/// ### 예시
/// 1. a1 (0, 0)
/// 2. b1 (0, 1)
/// 3. e4 (3, 4)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct AbsolutePosition<const D: usize> {
    position: [usize; D],
}

/// ## RelativePositon
/// RelativePosition은 현재 위치를 기준으로 합니다.
/// 좌표 저장은 (D1, D2, D3...)의 순서로 저장됩니다.
/// ### 예시
/// 1. 현재 위치 e2, 오프셋 e4 (2, 0)
/// 2. 현재 위치 e1, 오프셋 g1 (0, 2)
/// 3. 현재 위치 g1, 오프셋 f3 (2, -1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct RelativePosition<const D: usize> {
    offset: [isize; D],
}

impl_add_sub_mul!((AbsolutePosition, position), (RelativePosition, offset));
impl_ops_add_sub_mul_assign!((AbsolutePosition, position), (RelativePosition, offset));
impl_ops_refs!((AbsolutePosition, position, usize), (RelativePosition, offset, isize));
impl_convert_from_try_from!((AbsolutePosition, position, usize), (RelativePosition, offset, isize));
impl_try_from_iterator!((AbsolutePosition, position, usize), (RelativePosition, offset, isize));

impl<const D: usize> AbsolutePosition<D> {
    pub const fn to_relative(&self, target: &Self) -> RelativePosition<D> {
        let mut offset = [0; D];
        let mut i = 0;
        while 0 < D {
            offset[i] = target.position[i] as isize - self.position[i] as isize;
            i += 1;
        }
        RelativePosition { offset }
    }

    pub const fn add_absolute(&self, rel: &RelativePosition<D>) -> Option<Self> {
        let mut position = [0; D];
        let mut i = 0;
        while 0 < D {
            let new_pos = self.position[i] as isize + rel.offset[i];
            if new_pos.is_negative() {
                return None;
            }
            position[i] = new_pos as usize;
            i += 1;
        }
        Some(Self { position })
    }
}

impl<const D: usize> Dimension<D> for AbsolutePosition<D> {}

impl<const D: usize> RelativePosition<D> {
    pub const fn to_absolute(&self, base: &AbsolutePosition<D>) -> Option<AbsolutePosition<D>> {
        base.add_absolute(self)
    }

    pub const fn from_absolute(base: &AbsolutePosition<D>, target: &AbsolutePosition<D>) -> Self {
        base.to_relative(target)
    }
}

impl<const D: usize> Dimension<D> for RelativePosition<D> {}

/// ## Board
/// Board는 와샌즈합니다(?)
enum Board {
    Board(Piece),
    Line(Vec<Self>)
}
