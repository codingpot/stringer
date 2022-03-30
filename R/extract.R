str_extract <- function(l, x) {
    switch(
        class(x),
        text = str_text_extract(l, x@value),
        regex = str_regex_extract(l, x@value),
        character = str_regex_extract(l, x)
    )
}

str_extract_all <- function(l, x) {
    rs <- list()
    for (v in l) {
        r <- switch(
            class(x),
            text = single_str_text_extract_all(v, x@value),
            regex = single_str_regex_extract_all(v, x@value),
            character = single_str_regex_extract_all(v, x)
        )
        rs <- append(rs, r)
    }
    rs
}
