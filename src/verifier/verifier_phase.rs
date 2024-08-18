/// Directive or binding phase.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
#[repr(u32)]
pub enum VerifierPhase {
    Alpha = 0,
    Beta = 1,
    Delta = 2,
    Epsilon = 3,
    Eta = 4,
    Theta = 5,
    Omega = 6,
    Finished = 7,
}