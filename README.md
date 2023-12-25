# bible

A simple Rust program to generate Bible verses.

## Usage

```bash
./bible.sh
# Output:
# A random bible verse.
```

```bash
./bible.sh 0
# Output:
# Genesis 1:1
# In the beginning God created the heaven and the earth.
```

```bash
./bible.sh 31100
# Output:
# Revelation 22:20
# He which testifieth these things saith, Surely I come quickly. Amen. Even so, come, Lord Jesus.
```

```bash
./bible.sh 0 1
# Output:
# Genesis 1:1 - Genesis 1:2
# In the beginning God created the heaven and the earth. And the earth was without form, and void; and darkness was upon the face of the deep. And the Spirit of God moved upon the face of the waters.
```

```bash
./bible.sh -1
# Output:
# Revelation 22:21
# The grace of our Lord Jesus Christ be with you all. Amen.
```