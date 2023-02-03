#[derive(Copy, Clone, PartialEq)]
pub enum BranchType {
    /// Direct Branch with link
    DIRCALL,
    INDCALL, // Indirect Branch with link
    ERET, // Exception return (indirect)
    DBGEXIT, // Exit from Debug state
    RET, // Indirect branch with function return hint
    DIR, // Direct branch
    INDIR, // Indirect branch
    EXCEPTION, // Exception entry
    TMFAIL, // Transaction failure
    RESET, // Reset
    UNKNOWN
}