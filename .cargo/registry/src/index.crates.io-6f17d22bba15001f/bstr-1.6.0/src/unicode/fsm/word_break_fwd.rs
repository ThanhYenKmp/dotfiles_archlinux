// DO NOT EDIT THIS FILE. IT WAS AUTOMATICALLY GENERATED BY:
//
//     regex-cli generate serialize sparse dfa --minimize --start-kind anchored --shrink --rustfmt --safe WORD_BREAK_FWD src/unicode/fsm/ <snip: arg too long>
//
// regex-cli 0.0.1 is available on crates.io.

use regex_automata::{dfa::sparse::DFA, util::lazy::Lazy};

pub static WORD_BREAK_FWD: Lazy<DFA<&'static [u8]>> = Lazy::new(|| {
    #[cfg(target_endian = "big")]
    static BYTES: &'static [u8] =
        include_bytes!("word_break_fwd.bigendian.dfa");
    #[cfg(target_endian = "little")]
    static BYTES: &'static [u8] =
        include_bytes!("word_break_fwd.littleendian.dfa");
    let (dfa, _) =
        DFA::from_bytes(BYTES).expect("serialized DFA should be valid");
    dfa
});
