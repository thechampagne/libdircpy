/*
 * Copyright (c) 2022 XXIV
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::os::raw::c_int;
use std::ffi::CStr;
use std::slice;
use dircpy::copy_dir;
use dircpy::copy_dir_advanced;
use dircpy::CopyBuilder;

#[repr(C)]
struct dircpy_copy_builder_t {
  data: *mut c_void
}

/// Copy from 'source' directory to 'dest' directory
///
/// Example:
/// * *
/// int main() 
/// {
///   dircpy_copy_dir("src", "dest");
///   return 0;
/// }
/// * *
///
/// @param source
/// @param dest
/// @return 0 on success and non zero value on failure
#[no_mangle]
unsafe extern "C" fn dircpy_copy_dir(source: *const c_char, dest: *const c_char) -> c_int {
  let source_rs = match CStr::from_ptr(source).to_str() {
      Ok(s) => s,
      Err(_) => return -1,
  };
  let dest_rs = match CStr::from_ptr(dest).to_str() {
      Ok(s) => s,
      Err(_) => return -1,
  };

  match copy_dir(source_rs, dest_rs) {
    Ok(_) => 0,
    Err(_) => -1
  }
}

/// Copy from 'source' directory to 'dest' directory
///
/// Example:
/// * *
/// int main() 
/// {
///   char* exf[] = {};
///   char* inf[] = {};
///   dircpy_copy_dir_advanced("src",
///                            "dest",
///                             0, 0, 0, exf, 0, inf, 0);
///   return 0;
/// }
/// * *
///
/// @param source
/// @param dest
/// @param overwrite_all
/// @param overwrite_if_newer
/// @param overwrite_if_size_differs
/// @param exclude_filters
/// @param exclude_filters_length
/// @param include_filters
/// @param include_filters_length
/// @return 0 on success and non zero value on failure
#[no_mangle]
unsafe extern "C" fn dircpy_copy_dir_advanced(source: *const c_char,
                                              dest: *const c_char,
                                              overwrite_all: c_int,
                                              overwrite_if_newer: c_int,
                                              overwrite_if_size_differs: c_int,
                                              exclude_filters: *const *const c_char,
                                              exclude_filters_length: usize,
                                              include_filters: *const *const c_char,
                                              include_filters_length: usize) -> c_int {
  let source_rs = match CStr::from_ptr(source).to_str() {
      Ok(s) => s,
      Err(_) => return -1,
  };
  let dest_rs = match CStr::from_ptr(dest).to_str() {
      Ok(s) => s,
      Err(_) => return -1,
  };
  let exclude_filters_rs = slice::from_raw_parts(exclude_filters, exclude_filters_length).iter().map(|i| match CStr::from_ptr(*i).to_str() {
    Ok(s) => s.to_string(),
    Err(_) => "".to_string()
  } ).collect();
  let include_filters_rs = slice::from_raw_parts(include_filters, include_filters_length).iter().map(|i| match CStr::from_ptr(*i).to_str() {
    Ok(s) => s.to_string(),
    Err(_) => "".to_string()
  } ).collect();
  match copy_dir_advanced(source_rs, dest_rs,
                          if overwrite_all == 0 { false } else { true },
                          if overwrite_if_newer == 0 { false } else { true },
                          if overwrite_if_size_differs == 0 { false } else { true },
                          exclude_filters_rs,
                          include_filters_rs) {
    Ok(_) => 0,
    Err(_) => -1
  }
}

/// Initialize 'dircpy_copy_builder_t'
///
/// Example:
/// * *
/// int main() 
/// {
///   dircpy_copy_builder_t builder;
///   dircpy_copy_builder_new(&builder,"src", "dest");
///   return 0;
/// }
/// * *
///
/// @param copy_builder
/// @param source
/// @param dest
/// @return 0 on success and non zero value on failure
#[no_mangle]
unsafe extern "C" fn dircpy_copy_builder_new(copy_builder: *mut dircpy_copy_builder_t, source: *const c_char, dest: *const c_char) -> c_int {
  let source_rs = match CStr::from_ptr(source).to_str() {
    Ok(s) => s,
    Err(_) => return -1,
  };
  let dest_rs = match CStr::from_ptr(dest).to_str() {
    Ok(s) => s,
    Err(_) => return -1,
  };
  let res = CopyBuilder::new(source_rs, dest_rs);
  (*copy_builder).data = Box::into_raw(Box::new(res)) as *mut c_void;
  0
}

/// Overwrite target files (off by default)
///
/// Example:
/// * *
/// int main() 
/// {
///   dircpy_copy_builder_t builder;
///   dircpy_copy_builder_new(&builder,"src", "dest");
///   dircpy_copy_builder_overwrite(&builder, 0);
///   return 0;
/// }
/// * *
///
/// @param copy_builder
/// @param overwrite
/// @return 0 on success and non zero value on failure
#[no_mangle]
unsafe extern "C" fn dircpy_copy_builder_overwrite(copy_builder: *mut dircpy_copy_builder_t, overwrite: c_int) -> c_int {
  let builder = Box::from_raw((*copy_builder).data as *mut CopyBuilder);
  let res = builder.overwrite(if overwrite == 0 { false } else { true });
  (*copy_builder).data = Box::into_raw(Box::new(res)) as *mut c_void;
  0
}

/// Overwrite if the source is newer (off by default)
///
/// Example:
/// * *
/// int main() 
/// {
///   dircpy_copy_builder_t builder;
///   dircpy_copy_builder_new(&builder,"src", "dest");
///   dircpy_copy_builder_overwrite_if_newer(&builder, 0);
///   return 0;
/// }
/// * *
///
/// @param copy_builder
/// @param overwrite_only_newer
/// @return 0 on success and non zero value on failure
#[no_mangle]
unsafe extern "C" fn dircpy_copy_builder_overwrite_if_newer(copy_builder: *mut dircpy_copy_builder_t, overwrite_only_newer: c_int) -> c_int {
  let builder = Box::from_raw((*copy_builder).data as *mut CopyBuilder);
  let res = builder.overwrite_if_newer(if overwrite_only_newer == 0 { false } else { true });
  (*copy_builder).data = Box::into_raw(Box::new(res)) as *mut c_void;
  0
}

/// Overwrite if size between source and dest differs (off by default)
///
/// Example:
/// * *
/// int main() 
/// {
///   dircpy_copy_builder_t builder;
///   dircpy_copy_builder_new(&builder,"src", "dest");
///   dircpy_copy_builder_overwrite_if_size_differs(&builder, 0);
///   return 0;
/// }
/// * *
///
/// @param copy_builder
/// @param overwrite_if_size_differs
/// @return 0 on success and non zero value on failure
#[no_mangle]
unsafe extern "C" fn dircpy_copy_builder_overwrite_if_size_differs(copy_builder: *mut dircpy_copy_builder_t, overwrite_if_size_differs: c_int) -> c_int {
  let builder = Box::from_raw((*copy_builder).data as *mut CopyBuilder);
  let res = builder.overwrite_if_size_differs(if overwrite_if_size_differs == 0 { false } else { true });
  (*copy_builder).data = Box::into_raw(Box::new(res)) as *mut c_void;
  0
}

/// Do not copy files that contain this string
///
/// Example:
/// * *
/// int main() 
/// {
///   dircpy_copy_builder_t builder;
///   dircpy_copy_builder_new(&builder,"src", "dest");
///   dircpy_copy_builder_with_exclude_filter(&builder, "file");
///   return 0;
/// }
/// * *
///
/// @param copy_builder
/// @param f
/// @return 0 on success and non zero value on failure
#[no_mangle]
unsafe extern "C" fn dircpy_copy_builder_with_exclude_filter(copy_builder: *mut dircpy_copy_builder_t, f: *const c_char) -> c_int {
  let f_rs = match CStr::from_ptr(f).to_str() {
    Ok(s) => s,
    Err(_) => return -1,
  };
  let builder = Box::from_raw((*copy_builder).data as *mut CopyBuilder);
  let res = builder.with_exclude_filter(f_rs);
  (*copy_builder).data = Box::into_raw(Box::new(res)) as *mut c_void;
  0
}

/// Only copy files that contain this string.
///
/// Example:
/// * *
/// int main() 
/// {
///   dircpy_copy_builder_t builder;
///   dircpy_copy_builder_new(&builder,"src", "dest");
///   dircpy_copy_builder_with_include_filter(&builder, "file");
///   return 0;
/// }
/// * *
///
/// @param copy_builder
/// @param f
/// @return 0 on success and non zero value on failure
#[no_mangle]
unsafe extern "C" fn dircpy_copy_builder_with_include_filter(copy_builder: *mut dircpy_copy_builder_t, f: *const c_char) -> c_int {
  let f_rs = match CStr::from_ptr(f).to_str() {
    Ok(s) => s,
    Err(_) => return -1,
  };
  let builder = Box::from_raw((*copy_builder).data as *mut CopyBuilder);
  let res = builder.with_include_filter(f_rs);
  (*copy_builder).data = Box::into_raw(Box::new(res)) as *mut c_void;
  0
}

/// Execute the copy operation
///
/// Example:
/// * *
/// int main() 
/// {
///   dircpy_copy_builder_t builder;
///   dircpy_copy_builder_new(&builder,"src", "dest");
///   dircpy_copy_builder_run(&builder);
///   return 0;
/// }
/// * *
///
/// @param copy_builder
/// @return 0 on success and non zero value on failure
#[no_mangle]
unsafe extern "C" fn dircpy_copy_builder_run(copy_builder: *mut dircpy_copy_builder_t) -> c_int {
  let builder = &*((*copy_builder).data as *mut CopyBuilder);
  match builder.run() {
    Ok(_) => 0,
    Err(_) => -1
  }
}

/// function to free the memory after using dircpy_copy_builder_t
/// 
/// @param ptr pointer to dircpy_copy_builder_t
#[no_mangle]
unsafe extern "C" fn dircpy_copy_builder_free(ptr: *mut dircpy_copy_builder_t) {
  if !ptr.is_null() {
    _ = Box::from_raw((*ptr).data as *mut CopyBuilder);
  }
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn dircpy_copy_dir_test() {
    unsafe {
      assert_eq!(0, dircpy_copy_dir("src\0".as_ptr() as *const i8,"dest\0".as_ptr() as *const i8));
    }
  }

  #[test]
  fn dircpy_copy_dir_advanced_test() {
    unsafe {
      assert_eq!(0, dircpy_copy_dir_advanced("src\0".as_ptr() as *const i8,
                                             "dest2\0".as_ptr() as *const i8,
                                            0, 0, 0, [].as_ptr(), 0, [].as_ptr(), 0))
    }
  }

  #[test]
  fn dircpy_copy_builder_test() {
    unsafe {
      let mut builder = dircpy_copy_builder_t{
        data: 0 as *mut c_void
      };
      dircpy_copy_builder_new(&mut builder as *mut dircpy_copy_builder_t, "src\0".as_ptr() as *const i8, "dest3\0".as_ptr() as *const i8);
      dircpy_copy_builder_overwrite_if_newer(&mut builder as *mut dircpy_copy_builder_t, 0);
      dircpy_copy_builder_overwrite_if_size_differs(&mut builder as *mut dircpy_copy_builder_t, 0);
      dircpy_copy_builder_run(&mut builder as *mut dircpy_copy_builder_t);
      dircpy_copy_builder_free(&mut builder as *mut dircpy_copy_builder_t);
    }
  }
}