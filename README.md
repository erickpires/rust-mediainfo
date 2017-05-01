## About
This is a simple wrapper around the [MediaInfo](https://mediaarea.net/en/MediaInfo)
library using Rust types.

The entire libmediainfo API is not exposed yet. The reason is basically because I don't
have a test case for the rest of the API.

Some information about the MediaInfo API can be found
[here](https://mediaarea.net/en/MediaInfo/Support/SDK), but not a lot of information is
available. The *exemples* directory has some basic use of the API.

## Examples
The examples can be run with:

```
cargo run --example [example_name]
```

for example:

```
cargo run --example basic_info
```

## Documentation
Yet to come.

## Requiments
### MediaInfo Library
In Linux you should install libmediainfo (probably found in your distro package manager).
For Arch Linux users:

```
 # pacman -S libmediainfo
```

The wrapper was written against the version "0.7.94" (current version on Arch
repository).

We also depend on ```libc``` but it should be a dependency of ```libmediainfo```.

### pkg-config
Currently the build "script" only supports building with ```pkg-config```.
