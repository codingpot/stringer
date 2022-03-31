setClass(
    "fixed",
    representation(value = "character")
)

setClass(
    "regex",
    representation(value = "character")
)

fixed <- function(x) {
    new("fixed", value = x)
}

regex <- function(x) {
    new("regex", value = x)
}