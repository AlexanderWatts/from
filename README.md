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
element := html_tag '{' (element | attribute)* '}' | literal
html_tag := 'div' | 'span' | 'p' | 'form' | 'input' | 'button'
attribute := '@' literal '=' literal
literal := STRING
```

## How

From introduces a build step for creating frontend applications that transforms From code to JavaScript at compile time.

Take the following From code:

```
form {
    @method="POST"

    input {
	    @type="email"
	    @placeholder="Email"
    }

    input {
	    @type="password"
	    @placeholder="Password"
    }

    button {
	    @type="submit"
	    "Login"
    }
}
```

This is transformed into:

```js
function dom(target) {
    let form1 = element("form");
    let input2 = element("input");
    let input3 = element("input");
    let button4 = element("button");
    let t5 = literal("Login");
	attribute(form1, "method", "POST");
    attribute(input2, "type", "email");
    attribute(input2, "placeholder", "Email");
    attribute(input3, "type", "password");
    attribute(input3, "placeholder", "Password");
    attribute(button4, "type", "submit");
    append(target, form1);
    append(form1, input2);
    append(form1, input3);
    append(form1, button4);
    append(button4, t5);
}
```
The From runtime library provides the `element`, `literal`, `attribute` and `append` function declarations.

## Usage

The From CLI is responsible for reading .from files and outputting .js

```bash
# Outputs an io::Error or from.js file at "./"
from -i "<input.from>"

# Outputs a io::Error or js file at a given path
from -i "<input.from>" -o "<output.js>"
```
