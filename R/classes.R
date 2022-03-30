setClass(
    "text",
    representation(value = "character")
)

setClass(
    "regex",
    representation(value = "character")
)

text <- function(x) {
    new("text", value = x)
}

regex <- function(x) {
    new("regex", value = x)
}