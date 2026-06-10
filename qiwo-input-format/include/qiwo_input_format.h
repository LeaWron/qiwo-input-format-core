#ifndef QIWO_INPUT_FORMAT_H_
#define QIWO_INPUT_FORMAT_H_

#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct QiwoInputFormatOptions {
  bool auto_spacing_enabled;
} QiwoInputFormatOptions;

// Formats UTF-8 commit text and returns a newly allocated UTF-8 string.
//
// before_cursor and after_cursor are optional; pass NULL when surrounding text
// is unavailable. The returned pointer must be released with
// qiwo_input_format_free_string().
char* qiwo_input_format_commit_text(const char* commit_text,
                                    const char* before_cursor,
                                    const char* after_cursor,
                                    QiwoInputFormatOptions options);

void qiwo_input_format_free_string(char* value);

#ifdef __cplusplus
}
#endif

#endif  // QIWO_INPUT_FORMAT_H_
