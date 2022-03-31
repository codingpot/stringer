setClass(
    "fixed",
    representation(value = "character")
)

setClass(
    "regex",
    representation(value = "character")
)

setClass(
    "coll",
    representation(
        value = "character",
        locale = "character",
    )
)

fixed <- function(x) {
    new("fixed", value = x)
}

regex <- function(x) {
    new("regex", value = x)
}

coll <- function(x, local="en") {
    new("coll", value = x, locale = local)
}