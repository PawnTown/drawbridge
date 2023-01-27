use rlua::{Function, Lua, Result};

pub struct Middleware {
    lua_source: String,
}

impl Middleware {
    pub fn new(lua_source: String) -> Middleware {
        Middleware {
            lua_source: lua_source,
        }
    }

    fn apply_middleware(&self, fn_name: String, line: String) -> Result<String> {
        if &self.lua_source != "" {
            let lua = Lua::new();
            return lua.context(|lua_ctx| {
                lua_ctx.load(&self.lua_source).exec()?;

                let globals = lua_ctx.globals();

                let print: Function = globals.get(fn_name)?;
                let result = print.call::<_, String>(line)?;
                Ok(result)
            });
        }
        return Ok(line);
    }

    pub fn handle_in(&self, line: String) -> Result<String> {
        return self.apply_middleware("message_in".into(), line);
    }

    pub fn handle_out(&self, line: String) -> Result<String> {
        return self.apply_middleware("message_out".into(), line);
    }
}
