# From

From is the source-to-source compiler for the From DSL.

## Examples

```
div {
    @id="main"

    span {}

    div {
        @style="padding: 20px;"
        "Hello, 🌎!"
    }
}
```

## Grammar

```
program := element
element := ('div' | 'span') element_block | LITERAL
element_block := '{' (element | attribute)* '}'
attribute := '@' LITERAL '=' LITERAL
```

## How

From introduces a build step for creating frontend applications that transforms From code to JavaScript at compile time.

Take the following From code:

```
div {
    @class="flex"
    "Hello, 🌎!"
    span {}
}
```

This is transformed into:

```js
function dom() {
    return element(
        "div",
        {
            class: "flex"
        },
        literal("Hello, 🌎!"),
        element("span", {})
    )
}
```
The From runtime library provides the `element` and `literal` function declarations.
