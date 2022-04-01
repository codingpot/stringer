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

regex <- function(x, ignore_case = FALSE, multiline = FALSE, comments = FALSE, dot_all = FALSE) {
    new("regex", value = x, ignore_case = ignore_case, multiline = multiline, comments = comments, dot_all = dot_all)
}

coll <- function(x, ignore_case = FALSE, locale = "en") {
    new("coll", value = x, ignore_case = ignore_case, locale = locale)
}