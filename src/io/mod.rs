/// A bit is anything that can be in only one of two states at once.
///
/// We choose to represent a bit with a simple boolean and aliases for true and false (just for more convenient names).
type Bit = bool;
/// The "on" state of a bit. 
const ON: Bit = true;
/// The "off" state of a bit.
const OFF: Bit = false;


/// A wire is a basic unit containing only one bit at a time.
struct Wire(Bit);