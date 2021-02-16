use std::collections::HashMap;
use crate::stack::Item;

pub type State = HashMap<String, Data>;

#[allow(dead_code)]
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
    ($en:ident, $fun1:ident, $fun2:ident, $fun3:ident, $ret:ty) => {
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

        pub fn $fun3(this: Option<&Self>, def: $ret) -> $ret {
            if let Some($en(out)) = this {
                out.clone()
            } else {
                def
            }
        }
    }
}

macro_rules! as_num {
    ($en:ident, $fun1:ident, $fun2:ident, $type:ty) => {
        pub fn $fun1(&self) -> $type {
            if let $en(out) = self {
                (*out) as $type
            } else {
                panic!("Called Data::{} on non {} value", 
                    stringify!($fun),
                    stringify!($en));
            }
        }

        pub fn $fun2(this: Option<&Self>, def: $type) -> $type {
            if let Some($en(out)) = this {
                (*out) as $type
            } else {
                def
            }
        }
    }
}

#[allow(dead_code)]
impl Data {
    into_as!(Map  , into_state , as_state , unwrap_state_or , State);
    into_as!(Array, into_vec   , as_vec   , unwrap_vec_or   , Vec<Data>);
    into_as!(Str  , into_string, as_string, unwrap_string_or, String);

    pub fn from_u8   (n: u8   ) -> Data { Uint(n as u64) }
    pub fn from_u16  (n: u16  ) -> Data { Uint(n as u64) }
    pub fn from_u32  (n: u32  ) -> Data { Uint(n as u64) }
    pub fn from_usize(n: usize) -> Data { Uint(n as u64) }

    pub fn from_i8   (n: i8   ) -> Data { Int(n as i64) }
    pub fn from_i16  (n: i16  ) -> Data { Int(n as i64) }
    pub fn from_i32  (n: i32  ) -> Data { Int(n as i64) }
    pub fn from_isize(n: isize) -> Data { Int(n as i64) }

    pub fn from_f32  (n: f32  ) -> Data { Float(n as f64) }

    as_num!(Uint,  as_u8   , unwrap_u8_or   , u8   );
    as_num!(Uint,  as_u16  , unwrap_u16_or  , u16  );
    as_num!(Uint,  as_u32  , unwrap_u32_or  , u32  );
    as_num!(Uint,  as_u64  , unwrap_u64_or  , u64  );
    as_num!(Uint,  as_usize, unwrap_usize_or, usize);

    as_num!(Int,   as_i8   , unwrap_i8_or   , i8   );
    as_num!(Int,   as_i16  , unwrap_i16_or  , i16  );
    as_num!(Int,   as_i32  , unwrap_i32_or  , i32  );
    as_num!(Int,   as_i64  , unwrap_i64_or  , i64  );
    as_num!(Int,   as_isize, unwrap_isize_or, isize);

    as_num!(Float, as_f32  , unwrap_f32_or  , f32  );
    as_num!(Float, as_f64  , unwrap_f64_or  , f64  );

    as_num!(Bool,  as_bool , unwrap_bool_or , bool);
}
