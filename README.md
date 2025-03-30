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

```
form {
    @method="POST"
    @class="container mx-auto flex flex-col gap-5 items-center h-screen justify-center"

    input {
        @type="text"
        @class="border w-full md:w-1/2 p-2"
    }

    input {
        @type="password"
        @class="border w-full md:w-1/2 p-2"
    }

    button {
        @type="submit"
        @class="border p-2 cursor-pointer"
        "Submit"
    }
}
```

## Grammar

```
program := element
element := ('div'
    | 'span'
    | 'p'
    | 'form'
    | 'input'
    | 'button'
) element_block | LITERAL
element_block := '{' (element | attribute)* '}'
attribute := '@' LITERAL '=' LITERAL ';'
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

## Usage

The From CLI is responsible for processing .from files and outputting .js

```bash
# Outputs an io::Error or from.js file at "./"
from -i "<input>"

# Outputs a io::Error or js file at a given path
from -i "<input.from>" -o "<output.js>"
```
