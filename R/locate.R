str_locate <- function(l, x) {
    if (is.null(l) || is.null(x)) {
        return(NULL)
    }
    switch(
        class(x),
        fixed = str_bytes_locate(l, x@value),
        regex = str_regex_locate(l, x@value),
        coll = str_utf8_locate(l, x@value),
        character = str_regex_locate(l, x)
    )
}