use std::fs::Metadata;
use std::io;
use std::process::ExitCode;

use wasmtime::TypedFunc;

use rs_fsmetadata4mac::State;
use rs_fsmetadata4mac::StateBuilder;
use rs_fsmetadata4mac::WasmEngine;
use rs_fsmetadata4mac::WasmInstance;
use rs_fsmetadata4mac::WasmLinker;
use rs_fsmetadata4mac::WasmModule;
use rs_fsmetadata4mac::WasmStore;

fn sub() -> Result<(), io::Error> {
    let we: WasmEngine = WasmEngine::default();

    // Start the linker chain
    let mut wl: WasmLinker = we.create_linker4metadata();

    // --- Register Accessors (Metadata Retrieval) ---
    wl = wl.st_dev_wrap()?;
    wl = wl.st_ino_wrap()?;
    wl = wl.st_nlink_wrap()?;
    wl = wl.st_rdev_wrap()?;
    wl = wl.st_size_wrap()?;
    wl = wl.st_blksize_wrap()?;
    wl = wl.st_blocks_wrap()?;

    // Permissions/Flags (u32)
    wl = wl.st_mode_wrap()?;
    wl = wl.st_uid_wrap()?;
    wl = wl.st_gid_wrap()?;
    wl = wl.st_flags_wrap()?;
    wl = wl.st_gen_wrap()?;
    wl = wl.st_lspare_wrap()?;

    // Timestamps (i64)
    wl = wl.st_atime_wrap()?;
    wl = wl.st_atime_nsec_wrap()?;
    wl = wl.st_mtime_wrap()?;
    wl = wl.st_mtime_nsec_wrap()?;
    wl = wl.st_ctime_wrap()?;
    wl = wl.st_ctime_nsec_wrap()?;
    wl = wl.st_birthtime_wrap()?;
    wl = wl.st_birthtime_nsec_wrap()?;

    // --- Register Printers (Logging functions) ---
    wl = wl.print_st_dev_wrap()?;
    wl = wl.print_st_ino_wrap()?;
    wl = wl.print_st_size_wrap()?;
    wl = wl.print_st_nlink_wrap()?;
    wl = wl.print_st_rdev_wrap()?;
    wl = wl.print_st_blksize_wrap()?;
    wl = wl.print_st_blocks_wrap()?;

    // Permissions/Flags Printers
    wl = wl.print_st_mode_wrap()?;
    wl = wl.print_st_uid_wrap()?;
    wl = wl.print_st_gid_wrap()?;
    wl = wl.print_st_flags_wrap()?;
    wl = wl.print_st_gen_wrap()?;
    wl = wl.print_st_lspare_wrap()?;

    // Timestamps Printers
    wl = wl.print_st_atime_wrap()?;
    wl = wl.print_st_atime_nsec_wrap()?;
    wl = wl.print_st_mtime_wrap()?;
    wl = wl.print_st_mtime_nsec_wrap()?;
    wl = wl.print_st_ctime_wrap()?;
    wl = wl.print_st_ctime_nsec_wrap()?;
    wl = wl.print_st_birthtime_wrap()?;
    wl = wl.print_st_birthtime_nsec_wrap()?;

    // --- End of Linker Setup ---

    let path2file: String = std::env::var("ENV_PATH2FILE").unwrap_or_default();
    let met4file: Metadata = std::fs::metadata(path2file)?;

    let sbldr = StateBuilder::default()
        .limit_mem_size(1048576)
        .limit_num_instances(1);
    let state: State = sbldr.build(met4file);

    let path2wat: String = std::env::var("ENV_PATH2WAT").unwrap_or_default();
    let wat_bytes: Vec<u8> = std::fs::read(path2wat)?;

    let mdl: WasmModule = we.wat2module(&wat_bytes)?;

    let mut store: WasmStore = we.state2store(state);

    let ins: WasmInstance = wl.to_instance(&mut store, &mdl)?;

    let io_main: TypedFunc<(), ()> = ins.get_main(&mut store)?;

    io_main
        .call(store.as_store_mut(), ())
        .map_err(io::Error::other)?;

    Ok(())
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
