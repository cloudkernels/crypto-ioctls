use std::fs::File;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::os::raw::c_char;
use libc::{open, O_RDWR};

use vmm_sys_util::errno;
use vmm_sys_util::ioctl::{ioctl_with_ptr, ioctl_with_mut_ref};

use crate::ioctls::Result;
use crate::crypto_ioctls::*;
use crypto_bindings::*;

pub struct Crypto {
    crypto: File,
}

impl Crypto {
    /// Opens `/dev/crypto` and returns a `Crypto` object on success.
    ///
    /// # Arguments
    /// 
    /// * `device` - The absolute path to a crypto device
    ///
    /// # Example
    ///
    /// ```
    /// use crypto_ioctls::Crypto;
    /// let crypto = Crypto::new().unwrap();
    /// ```
    ///
    pub fn new(device: &str) -> Result<Self> {
        // Open device
        let ret = unsafe { open(device.as_ptr() as *const c_char, O_RDWR) };
        if ret < 0 {
            Err(errno::Error::last())
        } else {
            Ok(unsafe { Self::new_with_fd_number(ret) })
        }
    }

    /// Creates a new Crypto object assuming `fd` represents an existing open
    /// file descriptor associated with a crypto device such as `/dev/crypto`
    ///
    /// # Arguments
    ///
    /// * `fd` - File descriptor for a crypto device
    ///
    pub unsafe fn new_with_fd_number(fd: RawFd) -> Self {
        Crypto {
            crypto: File::from_raw_fd(fd),
        }
    }

    /// Opens a new Crypto session
    ///
    /// # Arguments
    /// 
    /// * `cfg` - Configuration of the new session.
    ///
    pub fn create_session(&self, cfg: &mut session_op) -> Result<()> {
        let ret = unsafe {
            ioctl_with_mut_ref(self, CIOCGSESSION(), cfg)
        };
        if ret < 0 {
            return Err(errno::Error::last());
        }
        Ok(())
    }

    /// Closes a Crypto session
    ///
    /// # Arguments
    ///
    /// * `sess_id` - Id of the session we are closing.
    ///
    pub fn close_session(&self, sess_id: u32) -> Result<()> {
        let ret = unsafe {
            ioctl_with_ptr(self, CIOCFSESSION(), &sess_id)
        };
        if ret < 0 {
            return Err(errno::Error::last());
        }
        Ok(())
    }

    /// Start a crypto operation
    ///
    /// # Arguments
    ///
    /// * `op` - The crypt_op struct describing the operation
    pub fn crypto_op(&self, op: &mut crypt_op) -> Result<()> {
        let ret = unsafe {
            ioctl_with_mut_ref(self, CIOCCRYPT(), op)
        };
        if ret < 0 {
            return Err(errno::Error::last());
        }
        Ok(())
    }
}

impl AsRawFd for Crypto {
    fn as_raw_fd(&self) -> RawFd {
        self.crypto.as_raw_fd()
    }
}
