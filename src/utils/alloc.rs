#[global_allocator]
pub static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
