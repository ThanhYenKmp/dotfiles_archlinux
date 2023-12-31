// DO NOT EDIT THIS FILE. IT WAS AUTOMATICALLY GENERATED BY:
//
//     regex-cli generate serialize dense dfa --minimize --start-kind anchored --reverse --no-captures --shrink --rustfmt --safe REGIONAL_INDICATOR_REV src/unicode/fsm/ \p{gcb=Regional_Indicator}
//
// regex-cli 0.0.1 is available on crates.io.

use regex_automata::{
    dfa::dense::DFA,
    util::{lazy::Lazy, wire::AlignAs},
};

pub static REGIONAL_INDICATOR_REV: Lazy<DFA<&'static [u32]>> =
    Lazy::new(|| {
        static ALIGNED: &AlignAs<[u8], u32> = &AlignAs {
            _align: [],
            #[cfg(target_endian = "big")]
            bytes: *include_bytes!("regional_indicator_rev.bigendian.dfa"),
            #[cfg(target_endian = "little")]
            bytes: *include_bytes!("regional_indicator_rev.littleendian.dfa"),
        };
        let (dfa, _) = DFA::from_bytes(&ALIGNED.bytes)
            .expect("serialized DFA should be valid");
        dfa
    });
