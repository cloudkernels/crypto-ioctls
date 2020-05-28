#[macro_use]
extern crate vmm_sys_util;

mod crypto_ioctls;
mod ioctls;

pub use ioctls::system::Crypto;
