use ilhook::x64::{
    CallbackOption, Hooker, HookFlags, HookPoint, HookType, JmpBackRoutine, RetnRoutine,
};

pub struct Interceptor {
    pub hooks: Vec<HookPoint>,
}

impl Interceptor {
    pub const fn new() -> Self {
        Self { hooks: Vec::new() }
    }

    #[allow(dead_code)]
    pub fn attach(
        &mut self,
        addr: usize,
        routine: JmpBackRoutine,
    ) -> Result<(), ilhook::HookError> {
        let hooker = Hooker::new(
            addr,
            HookType::JmpBack(routine),
            CallbackOption::None,
            0,
            HookFlags::empty(),
        );

        let hook_point = unsafe { hooker.hook() }?;
        self.hooks.push(hook_point);
        Ok(())
    }

    pub fn replace(
        &mut self,
        addr: usize,
        routine: RetnRoutine,
    ) -> Result<(), ilhook::HookError> {
        let hooker = Hooker::new(
            addr,
            HookType::Retn(routine),
            CallbackOption::None,
            0,
            HookFlags::empty(),
        );

        let hook_point = unsafe { hooker.hook() }?;
        self.hooks.push(hook_point);
        Ok(())
    }
}
