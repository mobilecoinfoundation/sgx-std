// Copyright (c) 2022 The MobileCoin Foundation
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/// The test enclave as bytes.
pub static ENCLAVE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/libenclave.signed.so"));
pub static ENCLAVE_KSS: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/libenclave_kss.signed.so"));
pub static ENCLAVE_PCL: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/libenclave_pcl.signed.so"));
pub static ENCLAVE_PCL_KEY: &[u8] = include_bytes!("pcl_key.bin");

use mc_sgx_core_sys_types::sgx_status_t;
use mc_sgx_urts_sys_types::sgx_enclave_id_t;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
