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

str_count <- function(l, pattern) {
    switch(
        class(pattern),
        text = str_text_count(l, pattern@value),
        regex = str_regex_count(l, pattern@value),
        character = str_regex_count(l, pattern)
    )
}