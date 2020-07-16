// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use crate::arrays::XorNameArray;
use ffi_utils::vec_from_raw_parts;
/// FFI-wrapper for `File`.
#[repr(C)]
pub struct File {
    /// File size in bytes.
    pub size: u64,
    /// Creation time (seconds part).
    pub created_sec: i64,
    /// Creation time (nanoseconds part).
    pub created_nsec: u32,
    /// Modification time (seconds part).
    pub modified_sec: i64,
    /// Modification time (nanoseconds part).
    pub modified_nsec: u32,
    /// Pointer to the user metadata.
    pub user_metadata: *const u8,
    /// Size of the user metadata.
    pub user_metadata_len: usize,
    /// Name of the `Blob` containing the content of this file.
    pub data_map_name: XorNameArray,
    /// Public status of the file
    pub published: bool,
}

impl Drop for File {
    fn drop(&mut self) {
        let _ =
            unsafe { vec_from_raw_parts(self.user_metadata as *mut u8, self.user_metadata_len) };
    }
}
