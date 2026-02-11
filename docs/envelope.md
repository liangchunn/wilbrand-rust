# `envelope.bin`

The `envelope.bin` file contains the image that is shown in the Wii Message Board.

## Requirements

- [`lzss`](https://github.com/leahanderson1/lzss)
- `wimgt` and `wszst` via [`szs`](https://szs.wiimm.de/download.html)

## Decoding/Encoding

The envelope looks like this:

```
U8Archive (
  LZSS (
    U8Archive (
      thumbnail_LZ.bin {
        img {
          my_LetterS_b.tpl
        }
      }
    }
  )
)
```

Extracting:

```sh
wszst extract envelope.bin
lzss -d envelope.d/thumbnail_LZ.bin
wszst extract envelope.d/thumbnail_LZ.bin
wimgt decode envelope.d/thumbnail_LZ.d/img/my_LetterS_b.tpl
```

Packing:

```sh
rm envelope.d/thumbnail_LZ.d/img/my_LetterS_b.tpl
# you can replace my_LetterS_b.tpl.png with your own png at this point
wimgt encode --transform TPL.CMPR envelope.d/thumbnail_LZ.d/img/my_LetterS_b.tpl.png
rm envelope.d/thumbnail_LZ.d/img/my_LetterS_b.tpl.png
wszst create envelope.d/thumbnail_LZ.d
rm -rf envelope.d/thumbnail_LZ.d
rm envelope.d/thumbnail_LZ.bin
mv envelope.d/thumbnail_LZ.u8 envelope.d/thumbnail_LZ.bin
lzss -evn envelope.d/thumbnail_LZ.bin
wszst create envelope.d
rm -rf envelope.d

# the output envelope.u8 is the packed archive
```

After generating the new `envelope.u8`, replace this line in `payload.rs` and update the slice size.
The compiler will prompt you for the correct values.

```diff
- const ENVELOPE: &[u8; 6304] = include_bytes!("../../data/envelope.bin");
+ const ENVELOPE: &[u8; 6816] = include_bytes!("../../data/envelope.u8");
```

## Source Code

- szs: https://github.com/Wiimm/wiimms-szs-tools
- lzss: https://github.com/leahanderson1/lzss
