# Welcome to md-viewer-rs

This is a **test file** for the TUI markdown viewer.

## Text Formatting

Here is some **bold text**, some *italic text*, and some `inline code`.
You can also combine **bold and *italic*** together.

Here is a [link to Rust](https://www.rust-lang.org/) documentation.

---

## Lists

### Unordered List

- First item
- Second item with **bold**
- Third item with `code`
  - Nested item one
  - Nested item two

### Ordered List

1. Step one
2. Step two
3. Step three

## Blockquotes

> This is a blockquote.
> It can span multiple lines.
>
> And have multiple paragraphs.

## Code Blocks

### Rust

```rust
fn main() {
    let greeting = "Hello, world!";
    println!("{}", greeting);

    for i in 0..5 {
        println!("Count: {}", i);
    }
}
```

### Python

```python
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

for i in range(10):
    print(f"fib({i}) = {fibonacci(i)}")
```

### JavaScript

```javascript
const fetchData = async (url) => {
  const response = await fetch(url);
  const data = await response.json();
  return data;
};
```

### JSON

```json
{
  "name": "md-viewer-rs",
  "version": "0.1.0",
  "features": ["syntax-highlighting", "file-browser", "scrolling"]
}
```

## Headings at Different Levels

### H3 Heading
#### H4 Heading
##### H5 Heading
###### H6 Heading

## Long Content for Scroll Testing

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris.

Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.

Curabitur pretium tincidunt lacus. Nulla gravida orci a odio. Nullam varius, turpis et commodo pharetra, est eros bibendum elit, nec luctus magna felis sollicitudin mauris.

Integer in mauris eu nibh euismod gravida. Duis ac tellus et risus vulputate vehicula. Donec lobortis risus a elit. Etiam tempor. Ut ullamcorper, ligula ut dictum pharetra, nisi nunc fringilla magna, in commodo elit erat nec turpis.

Praesent dapibus, neque id cursus faucibus, tortor neque egestas augue, eu vulputate magna eros eu erat. Aliquam erat volutpat. Nam dui mi, tincidunt quis, accumsan porttitor, facilisis luctus, metus.

---

*End of test file*
