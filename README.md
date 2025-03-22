# From

From is the source-to-source compiler for the From DSL.

## Examples

```
div {
    span {}
    div {
        "Hello, 🌎!"
    }
}
```

## Grammar

```
program := element
element := ('div' | 'span') element_block | LITERAL
element_block := '{' element* '}'
```

## How

From introduces a build step for creating frontend applications that transforms From code to JavaScript at compile time.

Take the following From code:

```
div {
    "Hello, 🌎!"
    span {}
}
```

This is transformed into:

```js
function dom() {
    return element("div",
        null,
        literal("Hello, 🌎!"),
        element("span", null)
    )
}
```
The From runtime library provides the `element` and `literal` function declarations.
