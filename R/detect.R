str_detect <- function(l, pattern, negate = FALSE) {
    result <- switch(
        class(pattern),
        text = str_text_detect(l, pattern@value),
        regex = str_regex_detect(l, pattern@value),
        character = str_regex_detect(l, pattern)
    )
    if (negate) {
        for (i in seq(length(result))) {
            result[i] <- !result[i]
        }
    }
    result
}