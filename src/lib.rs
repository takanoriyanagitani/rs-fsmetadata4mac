use std::fs::Metadata;
use std::io;
use std::os::darwin::fs::MetadataExt;

use wasmtime::Caller;
use wasmtime::Engine;
use wasmtime::Instance;
use wasmtime::Linker;
use wasmtime::Module;
use wasmtime::Store;
use wasmtime::TypedFunc;

use wasmtime::StoreLimits;
use wasmtime::StoreLimitsBuilder;

pub struct State {
    met: Metadata,
    lmt: StoreLimits,
}

impl State {
    pub fn st_dev(&self) -> u64 {
        self.met.st_dev()
    }
    pub fn st_ino(&self) -> u64 {
        self.met.st_ino()
    }
    pub fn st_nlink(&self) -> u64 {
        self.met.st_nlink()
    }
    pub fn st_rdev(&self) -> u64 {
        self.met.st_rdev()
    }
    pub fn st_size(&self) -> u64 {
        self.met.st_size()
    }
    pub fn st_blksize(&self) -> u64 {
        self.met.st_blksize()
    }
    pub fn st_blocks(&self) -> u64 {
        self.met.st_blocks()
    }

    pub fn st_mode(&self) -> u32 {
        self.met.st_mode()
    }
    pub fn st_uid(&self) -> u32 {
        self.met.st_uid()
    }
    pub fn st_gid(&self) -> u32 {
        self.met.st_gid()
    }
    pub fn st_flags(&self) -> u32 {
        self.met.st_flags()
    }
    pub fn st_gen(&self) -> u32 {
        self.met.st_gen()
    }
    pub fn st_lspare(&self) -> u32 {
        self.met.st_lspare()
    }

    pub fn st_atime(&self) -> i64 {
        self.met.st_atime()
    }
    pub fn st_atime_nsec(&self) -> i64 {
        self.met.st_atime_nsec()
    }
    pub fn st_mtime(&self) -> i64 {
        self.met.st_mtime()
    }
    pub fn st_mtime_nsec(&self) -> i64 {
        self.met.st_mtime_nsec()
    }
    pub fn st_ctime(&self) -> i64 {
        self.met.st_ctime()
    }
    pub fn st_ctime_nsec(&self) -> i64 {
        self.met.st_ctime_nsec()
    }
    pub fn st_birthtime(&self) -> i64 {
        self.met.st_birthtime()
    }
    pub fn st_birthtime_nsec(&self) -> i64 {
        self.met.st_birthtime_nsec()
    }
}

pub struct StateBuilder {
    lmt: StoreLimitsBuilder,
}

impl Default for StateBuilder {
    fn default() -> Self {
        Self {
            lmt: StoreLimitsBuilder::new(),
        }
    }
}

impl StateBuilder {
    pub fn limit_mem_size(self, limit_bytes: usize) -> Self {
        Self {
            lmt: self.lmt.memory_size(limit_bytes),
        }
    }

    pub fn limit_num_instances(self, limit: usize) -> Self {
        Self {
            lmt: self.lmt.instances(limit),
        }
    }
}

impl StateBuilder {
    pub fn build(self, met: Metadata) -> State {
        State {
            met,
            lmt: self.lmt.build(),
        }
    }
}

pub struct WasmStore(Store<State>);

impl WasmStore {
    pub fn as_store_mut(&mut self) -> &mut Store<State> {
        &mut self.0
    }
}

impl WasmStore {
    pub fn apply_limit(&mut self) {
        self.0.limiter(|state: &mut State| &mut state.lmt);
    }
}

#[derive(Default)]
pub struct WasmEngine(Engine);

impl WasmEngine {
    pub fn create_linker4metadata(&self) -> WasmLinker {
        WasmLinker(Linker::<State>::new(&self.0))
    }
}

pub struct WasmModule(Module);

impl WasmEngine {
    pub fn wat2module(&self, wat: &[u8]) -> Result<WasmModule, io::Error> {
        Module::new(&self.0, wat)
            .map_err(io::Error::other)
            .map(WasmModule)
    }
}

impl WasmEngine {
    pub fn linker4state(&self) -> Linker<State> {
        Linker::new(&self.0)
    }
}

impl WasmEngine {
    pub fn state2store(&self, state: State) -> WasmStore {
        let store = Store::new(&self.0, state);
        WasmStore(store)
    }
}

pub struct WasmLinker(Linker<State>);

impl WasmLinker {
    // --- Metadata/File System Accessors (u64 returns) ---

    pub fn st_dev_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "st_dev", |caller: Caller<'_, State>| {
            let dat: &State = caller.data();
            dat.met.st_dev()
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn st_ino_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "st_ino", |caller: Caller<'_, State>| {
            let dat: &State = caller.data();
            dat.met.st_ino()
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn st_nlink_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "st_nlink", |caller: Caller<'_, State>| {
            let dat: &State = caller.data();
            dat.met.st_nlink()
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn st_rdev_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "st_rdev", |caller: Caller<'_, State>| {
            let dat: &State = caller.data();
            dat.met.st_rdev()
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn st_size_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "st_size", |caller: Caller<'_, State>| {
            let dat: &State = caller.data();
            dat.met.st_size()
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn st_blksize_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "st_blksize", |caller: Caller<'_, State>| {
            let dat: &State = caller.data();
            dat.met.st_blksize()
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn st_blocks_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "st_blocks", |caller: Caller<'_, State>| {
            let dat: &State = caller.data();
            dat.met.st_blocks()
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    // --- Permissions/Flags Accessors (u32 returns) ---

    pub fn st_mode_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "st_mode", |caller: Caller<'_, State>| {
            let dat: &State = caller.data();
            dat.met.st_mode()
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn st_uid_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "st_uid", |caller: Caller<'_, State>| {
            let dat: &State = caller.data();
            dat.met.st_uid()
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn st_gid_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "st_gid", |caller: Caller<'_, State>| {
            let dat: &State = caller.data();
            dat.met.st_gid()
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn st_flags_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "st_flags", |caller: Caller<'_, State>| {
            let dat: &State = caller.data();
            dat.met.st_flags()
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn st_gen_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "st_gen", |caller: Caller<'_, State>| {
            let dat: &State = caller.data();
            dat.met.st_gen()
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn st_lspare_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "st_lspare", |caller: Caller<'_, State>| {
            let dat: &State = caller.data();
            dat.met.st_lspare()
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    // --- Timestamp Accessors (i64 returns) ---

    pub fn st_atime_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "st_atime", |caller: Caller<'_, State>| {
            let dat: &State = caller.data();
            dat.met.st_atime()
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn st_atime_nsec_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "st_atime_nsec", |caller: Caller<'_, State>| {
            let dat: &State = caller.data();
            dat.met.st_atime_nsec()
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn st_mtime_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "st_mtime", |caller: Caller<'_, State>| {
            let dat: &State = caller.data();
            dat.met.st_mtime()
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn st_mtime_nsec_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "st_mtime_nsec", |caller: Caller<'_, State>| {
            let dat: &State = caller.data();
            dat.met.st_mtime_nsec()
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn st_ctime_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "st_ctime", |caller: Caller<'_, State>| {
            let dat: &State = caller.data();
            dat.met.st_ctime()
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn st_ctime_nsec_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "st_ctime_nsec", |caller: Caller<'_, State>| {
            let dat: &State = caller.data();
            dat.met.st_ctime_nsec()
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn st_birthtime_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "st_birthtime", |caller: Caller<'_, State>| {
            let dat: &State = caller.data();
            dat.met.st_birthtime()
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn st_birthtime_nsec_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "st_birthtime_nsec", |caller: Caller<'_, State>| {
            let dat: &State = caller.data();
            dat.met.st_birthtime_nsec()
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }
}

impl WasmLinker {
    // --- Logging/Printing Wrappers  ---

    pub fn print_st_dev_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_st_dev",
            move |_: Caller<'_, State>, param: i64| {
                // Assuming the caller passes a u64 value as an i64
                let st_dev: u64 = param as u64;
                println!("st_dev: {}", st_dev);
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn print_st_ino_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_st_ino",
            move |_: Caller<'_, State>, param: i64| {
                let st_ino: u64 = param as u64;
                println!("st_ino: {}", st_ino);
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn print_st_size_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_st_size",
            move |_: Caller<'_, State>, param: i64| {
                let st_size: u64 = param as u64;
                println!("st_size: {}", st_size);
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    // u64 fields
    pub fn print_st_nlink_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_st_nlink",
            move |_: Caller<'_, State>, param: i64| {
                let st_nlink: u64 = param as u64;
                println!("st_nlink: {}", st_nlink);
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn print_st_rdev_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_st_rdev",
            move |_: Caller<'_, State>, param: i64| {
                let st_rdev: u64 = param as u64;
                println!("st_rdev: {}", st_rdev);
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn print_st_blksize_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_st_blksize",
            move |_: Caller<'_, State>, param: i64| {
                let st_blksize: u64 = param as u64;
                println!("st_blksize: {}", st_blksize);
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn print_st_blocks_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_st_blocks",
            move |_: Caller<'_, State>, param: i64| {
                let st_blocks: u64 = param as u64;
                println!("st_blocks: {}", st_blocks);
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    // u32 fields (Permissions/Flags)
    pub fn print_st_mode_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_st_mode",
            move |_: Caller<'_, State>, param: i32| {
                let st_mode: u32 = param as u32;
                println!("st_mode: {}", st_mode);
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn print_st_uid_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_st_uid",
            move |_: Caller<'_, State>, param: i32| {
                let st_uid: u32 = param as u32;
                println!("st_uid: {}", st_uid);
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn print_st_gid_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_st_gid",
            move |_: Caller<'_, State>, param: i32| {
                let st_gid: u32 = param as u32;
                println!("st_gid: {}", st_gid);
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn print_st_flags_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_st_flags",
            move |_: Caller<'_, State>, param: i32| {
                let st_flags: u32 = param as u32;
                println!("st_flags: {}", st_flags);
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn print_st_gen_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_st_gen",
            move |_: Caller<'_, State>, param: i32| {
                let st_gen: u32 = param as u32;
                println!("st_gen: {}", st_gen);
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn print_st_lspare_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_st_lspare",
            move |_: Caller<'_, State>, param: i32| {
                let st_lspare: u32 = param as u32;
                println!("st_lspare: {}", st_lspare);
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    // i64 fields (Timestamps)
    pub fn print_st_atime_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_st_atime",
            move |_: Caller<'_, State>, param: i64| {
                println!("st_atime: {}", param);
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn print_st_atime_nsec_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_st_atime_nsec",
            move |_: Caller<'_, State>, param: i64| {
                println!("st_atime_nsec: {}", param);
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn print_st_mtime_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_st_mtime",
            move |_: Caller<'_, State>, param: i64| {
                println!("st_mtime: {}", param);
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn print_st_mtime_nsec_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_st_mtime_nsec",
            move |_: Caller<'_, State>, param: i64| {
                println!("st_mtime_nsec: {}", param);
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn print_st_ctime_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_st_ctime",
            move |_: Caller<'_, State>, param: i64| {
                println!("st_ctime: {}", param);
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn print_st_ctime_nsec_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_st_ctime_nsec",
            move |_: Caller<'_, State>, param: i64| {
                println!("st_ctime_nsec: {}", param);
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn print_st_birthtime_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_st_birthtime",
            move |_: Caller<'_, State>, param: i64| {
                println!("st_birthtime: {}", param);
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn print_st_birthtime_nsec_wrap(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_st_birthtime_nsec",
            move |_: Caller<'_, State>, param: i64| {
                println!("st_birthtime_nsec: {}", param);
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }
}

pub struct WasmInstance(Instance);

impl WasmInstance {
    pub fn get_main(&self, store: &mut WasmStore) -> Result<TypedFunc<(), ()>, io::Error> {
        self.0
            .get_typed_func(&mut store.0, "io_main")
            .map_err(io::Error::other)
    }
}

impl WasmLinker {
    pub fn to_instance(
        &self,
        store: &mut WasmStore,
        mdl: &WasmModule,
    ) -> Result<WasmInstance, io::Error> {
        self.0
            .instantiate(&mut store.0, &mdl.0)
            .map_err(io::Error::other)
            .map(WasmInstance)
    }
}
