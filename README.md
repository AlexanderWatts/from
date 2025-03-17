# From

From is the source-to-source compiler for the From DSL.

## Examples

```
div {
	span {}
	div {}
}
```

## Grammar

```
program := element
element := ('div' | 'span') element_block
element_block := '{' element* '}'
```

## How

From introduces a build step for creating frontend applications that transforms From code to JavaScript at compile time.

Take the following From code:

```
div {
    div {}
    span {}
}
```

This is transformed into:

```js
function dom() {
    return element("div",
        element("span")
    )
}
```
The From runtime library provides the `element` function declaration responsible for rendering the elements.
