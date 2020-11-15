# yaml-split

yaml-split is a library which provides an iterator over individual YAML documents in a file or stream.

For example, you might have a YAML file like the following:

```
hello: world
---
foo: bar
```

This file contains two separate YAML documents. yaml-split will provide you the following two values in-order:

```
hello: world
```

```
---
foo: bar
```

This output is suitable for use by existing YAML deserializers such as [serde-yaml](https://github.com/dtolnay/serde-yaml).

## Usage

```
let file = File::open(f).unwrap();
let doc_iter = DocumentIterator::new(file);

for doc in doc_iter {
    println!("Doc:\n{}\n", doc);
}
```
