# skim-navi

A friendlier skim façade.


## Why?

[`skim`](https://github.com/lotabout/skim) is fiddly to use when i.e. navigating a tree hierarchy.

`skim-navi` addresses that.

## Usage

Implement the following async function and call `skim-navi::Navi::run()` with it:

```rust
pub async fn get_links(input: String) -> Result<Vec<String>, Error>
```

This represents the handler that gets called on each navigation update (i.e.: changing folders), and can even talk to a remote backend.

You can find a further examples in the [`examples`](https://github.com/mihaigalos/skim-navi/tree/main/examples) folder.
