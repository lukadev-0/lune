use core::fmt;
use std::ops;

use mlua::prelude::*;
use rbx_dom_weak::types::UDim as DomUDim;

use lune_utils::TableBuilder;

use crate::exports::LuaExportsTable;

use super::super::*;

/**
    An implementation of the [UDim](https://create.roblox.com/docs/reference/engine/datatypes/UDim) Roblox datatype.

    This implements all documented properties, methods & constructors of the `UDim` class as of October 2025.
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UDim {
    pub(crate) scale: f32,
    pub(crate) offset: i32,
}

impl UDim {
    pub(super) fn new(scale: f32, offset: i32) -> Self {
        Self { scale, offset }
    }
}

impl LuaExportsTable for UDim {
    const EXPORT_NAME: &'static str = "UDim";

    fn create_exports_table(lua: Lua) -> LuaResult<LuaTable> {
        let udim_new = |_: &Lua, (scale, offset): (Option<f32>, Option<i32>)| {
            Ok(UDim {
                scale: scale.unwrap_or_default(),
                offset: offset.unwrap_or_default(),
            })
        };

        TableBuilder::new(lua)?
            .with_function("new", udim_new)?
            .build_readonly()
    }
}

impl LuaUserData for UDim {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("Scale", |_, this| Ok(this.scale));
        fields.add_field_method_get("Offset", |_, this| Ok(this.offset));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Eq, userdata_impl_eq);
        methods.add_meta_method(LuaMetaMethod::ToString, userdata_impl_to_string);
        methods.add_meta_method(LuaMetaMethod::Unm, userdata_impl_unm);
        methods.add_meta_method(LuaMetaMethod::Add, userdata_impl_add);
        methods.add_meta_method(LuaMetaMethod::Sub, userdata_impl_sub);
    }
}

impl Default for UDim {
    fn default() -> Self {
        Self {
            scale: 0f32,
            offset: 0,
        }
    }
}

impl fmt::Display for UDim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.scale, self.offset)
    }
}

impl ops::Neg for UDim {
    type Output = Self;
    fn neg(self) -> Self::Output {
        UDim {
            scale: -self.scale,
            offset: -self.offset,
        }
    }
}

impl ops::Add for UDim {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        UDim {
            scale: self.scale + rhs.scale,
            offset: self.offset + rhs.offset,
        }
    }
}

impl ops::Sub for UDim {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        UDim {
            scale: self.scale - rhs.scale,
            offset: self.offset - rhs.offset,
        }
    }
}

impl From<DomUDim> for UDim {
    fn from(v: DomUDim) -> Self {
        UDim {
            scale: v.scale,
            offset: v.offset,
        }
    }
}

impl From<UDim> for DomUDim {
    fn from(v: UDim) -> Self {
        DomUDim {
            scale: v.scale,
            offset: v.offset,
        }
    }
}
