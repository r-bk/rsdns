/// Recursive query configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Recursion {
    /// Non-recursive query is sent to a nameserver.
    Off,

    /// Recursive query is sent to a nameserver.
    ///
    /// This is the default behavior.
    On,
}
