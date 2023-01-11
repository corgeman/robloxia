# robloxia
My first ever Rust program meant to bruteforce the AES encryption that https://aesencryption.net/ uses. Named "Robloxia" after the 3 billion Roblox puzzles using this stupid website. 
### https://aesencryption.net/ specifications
It's pretty similar to the PHP code at the bottom of the site. It encrypts/decrypts with AES in CBC mode using the constant IV of `12345678b0z2345n` and pads with PKCS#7. The key is generated without any hashing, it just shortens your password or pads it with null bytes to fit the length of whatever keylength you request it.
### The program
- Asks for base64-encoded ciphertext
- Asks for a wordlist to perform a dictionary attack with. I would reccomend [ignis-10m](https://weakpass.com/wordlist/1935) for a small attack, and [weakpass_3p](https://weakpass.com/wordlist/1949) for a big attack.
- Asks for the keysize. If you don't know, give it `911` and it'll brute 128/192/256.
### Program Speed
It currently runs at ~2 million guesses/second on my CPU utilizing the AES-NI instructions it comes with.
### Program Issues
- In order to eliminate possible keys, the program checks if the plaintext is valid UTF-8. This should work in 99% of cases but obviously won't work if your plaintext isn't valid UTF-8. If it's not, I don't know how to help you.
- This definitely isn't as optimized as it only uses 10% of my CPU
- Some wordlists will crash the program due to encoding issues (expects UTF-8)
### Compilation
I believe compilation is as simple as:
- `git clone https://github.com/corgeman/robloxia.git`
- `cd robloxia`
- `cargo build --release`


Inside ./robloxia/target/release should be robloxia.exe, the compiled binary.


If your computer's CPU supports AES-NI, then you can confirm the binary uses those instructions by doing:

Windows:
- `SET RUSTFLAGS=-Ctarget-feature=+aes,+ssse3`
- `cargo build --release`
- `SET RUSTFLAGS=`


Linux:
- `RUSTFLAGS="-Ctarget-feature=+aes,+ssse3" cargo build --release`
