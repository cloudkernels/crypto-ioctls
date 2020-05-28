//! Declares the Crypto ioctl 

use crypto_bindings::*;
use crate::ioctls::*;

// Ioctls for a cryptodev device

ioctl_iow_nr!(CRIOGET, CRYPTOIO, 101, u32);
ioctl_iowr_nr!(CIOCGSESSION, CRYPTOIO, 102, session_op);
ioctl_iow_nr!(CIOCFSESSION, CRYPTOIO, 103, u32);
ioctl_iowr_nr!(CIOCCRYPT, CRYPTOIO, 104, crypt_op);
ioctl_iowr_nr!(CIOCKEY, CRYPTOIO, 105, crypt_kop);
ioctl_ior_nr!(CIOCASYMFEAT, CRYPTOIO, 106, u32);
ioctl_iowr_nr!(CIOCGSESSINFO, CRYPTOIO, 107, session_info_op);
