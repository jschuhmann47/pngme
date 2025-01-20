# PNGme

Save secret messages in your .png files!

```
PNGs with messages

Usage: pngme <COMMAND>

Commands:
  encode  Encodes a message in a PNG file
  decode  Decodes a message in a PNG file
  remove  Removes a chunk type from a PNG file
  print   Prints message from a PNG file
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

For example, I encoded the message _world_ using the chunk _heLo_:
```
Png: { header: [137, 80, 78, 71, 13, 10, 26, 10], chunks: ["length: 13, type: IHDR, data: \"non utf-8\", crc: 3275645387", "length: 8192, type: IDAT, data: \"non utf-8\", crc: 3793648251", "length: 2983, type: IDAT, data: \"non utf-8\", crc: 2006393086", "length: 0, type: IEND, data: \"\", crc: 2923585666", "length: 16, type: heLo, data: \"world\", crc: 2441798988"] }
```

Made using this [guide](https://jrdngr.github.io/pngme_book/)
