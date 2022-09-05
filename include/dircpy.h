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
#ifndef __DIRCPY_H__
#define __DIRCPY_H__

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct dircpy_copy_builder_t dircpy_copy_builder_t;

/*
* Copy from 'source' directory to 'dest' directory
*
* Example:
* * *
* int main() 
* {
*   dircpy_copy_dir("src", "dest");
*   return 0;
* }
* * *
*
* @param source
* @param dest
* @return 0 on success and non zero value on failure
*/
extern int dircpy_copy_dir(const char* source, const char* dest);

/*
* Copy from 'source' directory to 'dest' directory
*
* Example:
* * *
* int main() 
* {
*   char* exf[] = {};
*   char* inf[] = {};
*   dircpy_copy_dir_advanced("src",
*                            "dest",
*                             0, 0, 0, exf, 0, inf, 0);
*   return 0;
* }
* * *
*
* @param source
* @param dest
* @param overwrite_all
* @param overwrite_if_newer
* @param overwrite_if_size_differs
* @param exclude_filters
* @param exclude_filters_length
* @param include_filters
* @param include_filters_length
* @return 0 on success and non zero value on failure
*/
extern int dircpy_copy_dir_advanced(const char* source,
                                    const char* dest,
                                    int overwrite_all,
                                    int overwrite_if_newer,
                                    int overwrite_if_size_differs,
                                    const char* const* exclude_filters,
                                    size_t exclude_filters_length,
                                    const char* const* include_filters,
                                    size_t include_filters_length);

/*
* Initialize 'dircpy_copy_builder_t'
*
* Example:
* * *
* int main() 
* {
*   dircpy_copy_builder_t builder;
*   dircpy_copy_builder_new(&builder,"src", "dest");
*   return 0;
* }
* * *
*
* @param copy_builder
* @param source
* @param dest
* @return 0 on success and non zero value on failure
*/
extern int dircpy_copy_builder_new(dircpy_copy_builder_t* copy_builder, const char* source, const char* dest);

/*
* Overwrite target files (off by default)
*
* Example:
* * *
* int main() 
* {
*   dircpy_copy_builder_t builder;
*   dircpy_copy_builder_new(&builder,"src", "dest");
*   dircpy_copy_builder_overwrite(&builder, 0);
*   return 0;
* }
* * *
*
* @param copy_builder
* @param overwrite
* @return 0 on success and non zero value on failure
*/
extern int dircpy_copy_builder_overwrite(dircpy_copy_builder_t* copy_builder, int overwrite);

/*
* Overwrite if the source is newer (off by default)
*
* Example:
* * *
* int main() 
* {
*   dircpy_copy_builder_t builder;
*   dircpy_copy_builder_new(&builder,"src", "dest");
*   dircpy_copy_builder_overwrite_if_newer(&builder, 0);
*   return 0;
* }
* * *
*
* @param copy_builder
* @param overwrite_only_newer
* @return 0 on success and non zero value on failure
*/
extern int dircpy_copy_builder_overwrite_if_newer(dircpy_copy_builder_t* copy_builder, int overwrite_only_newer);

/*
* Overwrite if size between source and dest differs (off by default)
*
* Example:
* * *
* int main() 
* {
*   dircpy_copy_builder_t builder;
*   dircpy_copy_builder_new(&builder,"src", "dest");
*   dircpy_copy_builder_overwrite_if_size_differs(&builder, 0);
*   return 0;
* }
* * *
*
* @param copy_builder
* @param overwrite_if_size_differs
* @return 0 on success and non zero value on failure
*/
extern int dircpy_copy_builder_overwrite_if_size_differs(dircpy_copy_builder_t* copy_builder, int overwrite_if_size_differs);

/*
* Do not copy files that contain this string
*
* Example:
* * *
* int main() 
* {
*   dircpy_copy_builder_t builder;
*   dircpy_copy_builder_new(&builder,"src", "dest");
*   dircpy_copy_builder_with_exclude_filter(&builder, "file");
*   return 0;
* }
* * *
*
* @param copy_builder
* @param f
* @return 0 on success and non zero value on failure
*/
extern int dircpy_copy_builder_with_exclude_filter(dircpy_copy_builder_t* copy_builder, const char* f);

/*
* Only copy files that contain this string.
*
* Example:
* * *
* int main() 
* {
*   dircpy_copy_builder_t builder;
*   dircpy_copy_builder_new(&builder,"src", "dest");
*   dircpy_copy_builder_with_include_filter(&builder, "file");
*   return 0;
* }
* * *
*
* @param copy_builder
* @param f
* @return 0 on success and non zero value on failure
*/
extern int dircpy_copy_builder_with_include_filter(dircpy_copy_builder_t* copy_builder, const char* f);

/*
* Execute the copy operation
*
* Example:
* * *
* int main() 
* {
*   dircpy_copy_builder_t builder;
*   dircpy_copy_builder_new(&builder,"src", "dest");
*   dircpy_copy_builder_run(&builder);
*   return 0;
* }
* * *
*
* @param copy_builder
* @return 0 on success and non zero value on failure
*/
extern int dircpy_copy_builder_run(dircpy_copy_builder_t* copy_builder);

/*
* function to free the memory after using dircpy_copy_builder_t
* 
* @param ptr pointer to dircpy_copy_builder_t
*/
extern void dircpy_copy_builder_free(dircpy_copy_builder_t* ptr);

#ifdef __cplusplus
}
#endif

#endif // __DIRCPY_H__