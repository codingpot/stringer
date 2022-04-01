setClass(
    "fixed",
    representation(
        value = "character",
        ignore_case = "logical"
    )
)

setClass(
    "regex",
    representation(
        value = "character",
        ignore_case = "logical",
        multiline = "logical",
        dot_all = "logical",
        comments = "logical"
    )
)

setClass(
    "coll",
    representation(
        value = "character",
        ignore_case = "logical",
        locale = "character"
    )
)

fixed <- function(x, ignore_case = FALSE) {
    new("fixed", value = x, ignore_case = ignore_case)
}

get_fixed <- function(x) {
    if (is.null(x)) {
        return(NULL)
    }

    if (x@ignore_case) {
        x@value <- convert_to_uppercase(x@value)
    }

    x@value
}

regex <- function(x, ignore_case = FALSE, multiline = FALSE, comments = FALSE, dot_all = FALSE) {
    new("regex", value = x, ignore_case = ignore_case, multiline = multiline, comments = comments, dot_all = dot_all)
}

get_regex <- function(x) {
    if (is.null(x)) {
        return(NULL)
    }

    if (x@dot_all) {
        x@value <- add_s_flag_to_dot(x@value)
    }

    if (x@multiline) {
        x@value <- add_m_flag_to(x@value)
    }

    if (x@comments) {
        x@value <- add_x_flag_to(x@value)
    }

    if (x@ignore_case) {
        x@value <- add_i_flag_to(x@value)
    }

    x@value
}

coll <- function(x, ignore_case = FALSE, locale = "en") {
    new("coll", value = x, ignore_case = ignore_case, locale = locale)
}

get_coll <- function(x) {
    if (is.null(x)) {
        return(NULL)
    }

    if (x@ignore_case) {
        x@value <- convert_to_uppercase(x@value)
    }

    x@value
}
