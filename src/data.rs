use std::collections::HashMap;
use crate::stack::Item;

pub type State = HashMap<String, Data>;

#[derive(Clone, Debug)]
pub enum Data {
    Map(State),
    Array(Vec<Data>),
    Float(f64),
    Bool(bool),
    Int(i64),
    Uint(u64),
    Str(String),
    Itm(Item)
}

pub use Data::*;

macro_rules! into_as {
    ($en:ident, $fun1:ident, $fun2: ident, $ret:ty) => {
        pub fn $fun1(self) -> $ret {
            if let $en(out) = self {
                out
            } else {
                panic!("Called Data::{} on non {} value", 
                    stringify!($fun1),
                    stringify!($en));
            }
        }

        pub fn $fun2(&self) -> $ret {
            if let $en(out) = &self {
                out.clone()
            } else {
                panic!("Called Data::{} on non {} value", 
                    stringify!($fun2),
                    stringify!($en));
            }
        }
    }
}

macro_rules! as_num {
    ($en:ident, $fun:ident, $type:ty) => {
        pub fn $fun(&self) -> $type {
            if let $en(out) = self {
                (*out) as $type
            } else {
                panic!("Called Data::{} on non {} value", 
                    stringify!($fun),
                    stringify!($en));
            }
        }
    }
}

#[allow(dead_code)]
impl Data {
    into_as!(Map, into_state, as_state, State);
    into_as!(Array, into_vec, as_vec, Vec<Data>);
    into_as!(Str, into_string, as_string, String);

    pub fn from_u8   (n: u8   ) -> Data { Uint(n as u64) }
    pub fn from_u16  (n: u16  ) -> Data { Uint(n as u64) }
    pub fn from_u32  (n: u32  ) -> Data { Uint(n as u64) }
    pub fn from_usize(n: usize) -> Data { Uint(n as u64) }

    pub fn from_i8   (n: i8   ) -> Data { Int(n as i64) }
    pub fn from_i16  (n: i16  ) -> Data { Int(n as i64) }
    pub fn from_i32  (n: i32  ) -> Data { Int(n as i64) }
    pub fn from_isize(n: isize) -> Data { Int(n as i64) }

    pub fn from_f32  (n: f32  ) -> Data { Float(n as f64) }

    as_num!(Uint,  as_u8   , u8   );
    as_num!(Uint,  as_u16  , u16  );
    as_num!(Uint,  as_u32  , u32  );
    as_num!(Uint,  as_usize, usize);

    as_num!(Int,   as_i8   , i8   );
    as_num!(Int,   as_i16  , i16  );
    as_num!(Int,   as_i32  , i32  );
    as_num!(Int,   as_isize, isize);

    as_num!(Float, as_f32  , f32  );
}
