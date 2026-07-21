(module
  ;; --- Imports from host ---

  ;; u64 fields (Imported as i64)
  (import "host" "st_dev" (func $st_dev (result i64)))
  (import "host" "st_ino" (func $st_ino (result i64)))
  (import "host" "st_nlink" (func $st_nlink (result i64)))
  (import "host" "st_rdev" (func $st_rdev (result i64)))
  (import "host" "st_size" (func $st_size (result i64)))
  (import "host" "st_blksize" (func $st_blksize (result i64)))
  (import "host" "st_blocks" (func $st_blocks (result i64)))

  ;; u32 fields (Imported as i32)
  (import "host" "st_mode" (func $st_mode (result i32)))
  (import "host" "st_uid" (func $st_uid (result i32)))
  (import "host" "st_gid" (func $st_gid (result i32)))
  (import "host" "st_flags" (func $st_flags (result i32)))
  (import "host" "st_gen" (func $st_gen (result i32)))
  (import "host" "st_lspare" (func $st_lspare (result i32)))

  ;; i64 timestamps
  (import "host" "st_atime" (func $st_atime (result i64)))
  (import "host" "st_atime_nsec" (func $st_atime_nsec (result i64)))
  (import "host" "st_mtime" (func $st_mtime (result i64)))
  (import "host" "st_mtime_nsec" (func $st_mtime_nsec (result i64)))
  (import "host" "st_ctime" (func $st_ctime (result i64)))
  (import "host" "st_ctime_nsec" (func $st_ctime_nsec (result i64)))
  (import "host" "st_birthtime" (func $st_birthtime (result i64)))
  (import "host" "st_birthtime_nsec" (func $st_birthtime_nsec (result i64)))

  ;; Printers for u64 fields
  (import "host" "print_st_dev" (func $print_st_dev (param i64)))
  (import "host" "print_st_ino" (func $print_st_ino (param i64)))
  (import "host" "print_st_nlink" (func $print_st_nlink (param i64)))
  (import "host" "print_st_rdev" (func $print_st_rdev (param i64)))
  (import "host" "print_st_size" (func $print_st_size (param i64)))
  (import "host" "print_st_blksize" (func $print_st_blksize (param i64)))
  (import "host" "print_st_blocks" (func $print_st_blocks (param i64)))

  ;; Printers for u32 fields
  (import "host" "print_st_mode" (func $print_st_mode (param i32)))
  (import "host" "print_st_uid" (func $print_st_uid (param i32)))
  (import "host" "print_st_gid" (func $print_st_gid (param i32)))
  (import "host" "print_st_flags" (func $print_st_flags (param i32)))
  (import "host" "print_st_gen" (func $print_st_gen (param i32)))
  (import "host" "print_st_lspare" (func $print_st_lspare (param i32)))

  ;; Printers for timestamps
  (import "host" "print_st_atime" (func $print_st_atime (param i64)))
  (import "host" "print_st_atime_nsec" (func $print_st_atime_nsec (param i64)))
  (import "host" "print_st_mtime" (func $print_st_mtime (param i64)))
  (import "host" "print_st_mtime_nsec" (func $print_st_mtime_nsec (param i64)))
  (import "host" "print_st_ctime" (func $print_st_ctime (param i64)))
  (import "host" "print_st_ctime_nsec" (func $print_st_ctime_nsec (param i64)))
  (import "host" "print_st_birthtime" (func $print_st_birthtime (param i64)))
  (import "host" "print_st_birthtime_nsec" (func $print_st_birthtime_nsec (param i64)))

  (func $io_main (export "io_main")
    ;; Local variables for u64/i64 values
    (local $v_dev i64) (local $v_ino i64) (local $v_nlink i64) 
    (local $v_rdev i64) (local $v_size i64) (local $v_blksize i64) (local $v_blocks i64)
    (local $v_atime i64) (local $v_atime_ns i64) (local $v_mtime i64) 
    (local $v_mtime_ns i64) (local $v_ctime i64) (local $v_ctime_ns i64) 
    (local $v_birth i64) (local $v_birth_ns i64)

    ;; Local variables for u32/i32 values
    (local $v_mode i32) (local $v_uid i32) (local $v_gid i32) 
    (local $v_flags i32) (local $v_gen i32) (local $v_lspare i32)

    ;; --- u64 Get & Print ---
    call $st_dev local.set $v_dev       local.get $v_dev call $print_st_dev
    call $st_ino local.set $v_ino       local.get $v_ino call $print_st_ino
    call $st_nlink local.set $v_nlink   local.get $v_nlink call $print_st_nlink
    call $st_rdev local.set $v_rdev     local.get $v_rdev call $print_st_rdev
    call $st_size local.set $v_size     local.get $v_size call $print_st_size
    call $st_blksize local.set $v_blksize local.get $v_blksize call $print_st_blksize
    call $st_blocks local.set $v_blocks local.get $v_blocks call $print_st_blocks

    ;; --- u32 Get & Print ---
    call $st_mode local.set $v_mode     local.get $v_mode call $print_st_mode
    call $st_uid local.set $v_uid       local.get $v_uid call $print_st_uid
    call $st_gid local.set $v_gid       local.get $v_gid call $print_st_gid
    call $st_flags local.set $v_flags   local.get $v_flags call $print_st_flags
    call $st_gen local.set $v_gen       local.get $v_gen call $print_st_gen
    call $st_lspare local.set $v_lspare local.get $v_lspare call $print_st_lspare

    ;; --- Timestamps Get & Print ---
    call $st_atime local.set $v_atime       local.get $v_atime call $print_st_atime
    call $st_atime_nsec local.set $v_atime_ns local.get $v_atime_ns call $print_st_atime_nsec
    call $st_mtime local.set $v_mtime       local.get $v_mtime call $print_st_mtime
    call $st_mtime_nsec local.set $v_mtime_ns local.get $v_mtime_ns call $print_st_mtime_nsec
    call $st_ctime local.set $v_ctime       local.get $v_ctime call $print_st_ctime
    call $st_ctime_nsec local.set $v_ctime_ns local.get $v_ctime_ns call $print_st_ctime_nsec
    call $st_birthtime local.set $v_birth   local.get $v_birth call $print_st_birthtime
    call $st_birthtime_nsec local.set $v_birth_ns local.get $v_birth_ns call $print_st_birthtime_nsec
  )
)
