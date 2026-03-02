use strum::EnumIter;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum Prototype {
    KingOwnerMask,
    KingFactionMask,
    QueenOwnerMask,
    QueenFactionMask,
    RookOwnerMask,
    RookFactionMask,
    BishopOwnerMask,
    BishopFactionMask,
    KnightOwnerMask,
    KnightFactionMask,
    PawnOwnerMask,
    PawnFactionMask,
}
