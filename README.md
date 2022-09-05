# libdircpy

[![](https://img.shields.io/github/v/tag/thechampagne/libdircpy?label=version)](https://github.com/thechampagne/libdircpy/releases/latest) [![](https://img.shields.io/github/license/thechampagne/libdircpy)](https://github.com/thechampagne/libdircpy/blob/main/LICENSE)

A **C** library to recursively copy directories.

### Installation & Setup

#### 1. Clone the repository
```
git clone https://github.com/thechampagne/libdircpy.git
```
#### 2. Navigate to the root
```
cd libdircpy
```
#### 3.1 Build the project
```
cargo build
```
#### 3.2 Run tests
```
cargo test
```

### API

```c
typedef struct dircpy_copy_builder_t dircpy_copy_builder_t;

int dircpy_copy_dir(const char* source, const char* dest);

int dircpy_copy_dir_advanced(const char* source,
                                    const char* dest,
                                    int overwrite_all,
                                    int overwrite_if_newer,
                                    int overwrite_if_size_differs,
                                    const char* const* exclude_filters,
                                    size_t exclude_filters_length,
                                    const char* const* include_filters,
                                    size_t include_filters_length);

int dircpy_copy_builder_new(dircpy_copy_builder_t* copy_builder, const char* source, const char* dest);

int dircpy_copy_builder_overwrite(dircpy_copy_builder_t* copy_builder, int overwrite);

int dircpy_copy_builder_overwrite_if_newer(dircpy_copy_builder_t* copy_builder, int overwrite_only_newer);

int dircpy_copy_builder_overwrite_if_size_differs(dircpy_copy_builder_t* copy_builder, int overwrite_if_size_differs);

int dircpy_copy_builder_with_exclude_filter(dircpy_copy_builder_t* copy_builder, const char* f);

int dircpy_copy_builder_with_include_filter(dircpy_copy_builder_t* copy_builder, const char* f);

int dircpy_copy_builder_run(dircpy_copy_builder_t* copy_builder);

void dircpy_copy_builder_free(dircpy_copy_builder_t* ptr);
```

### References
 - [dircpy](https://github.com/woelper/dircpy)

### License

This repo is released under the [MIT](https://github.com/thechampagne/libdircpy/blob/main/LICENSE).
