str_locate_single <- function(l, x) {
    if (is.null(l) || is.null(x)) {
        return(NULL)
    }

    if (class(x) == "fixed" || class(x) == "regex" && x@ignore_case) {
        l <- convert_to_uppercase(l)
    }

    switch(
        class(x),
        fixed = str_bytes_locate(l, get_fixed(x)),
        regex = str_regex_locate(l, get_regex(x)),
        coll = str_utf8_locate(l, get_coll(x), x@ignore_case),
        character = str_regex_locate(l, x)
    )
}