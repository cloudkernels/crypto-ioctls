use vmm_sys_util::errno;

pub const CRYPTOIO : u32 = 0x63; 

pub type Result<T> = std::result::Result<T, errno::Error>;

pub mod system;
