str_count <- function(l, pattern) {
    switch(
        class(pattern),
        fixed = str_text_count(l, pattern@value),
        regex = str_regex_count(l, pattern@value),
        character = str_regex_count(l, pattern)
    )
}